use std::collections::HashMap;
use crate::message::Messages;
use crate::storage::Storage;

const PSEUDO_INSTRUCTIONS: &[&str] = &["ORG", "HEX", "DEC", "END"];

#[derive(Debug)]
struct ParsedInstruction<'a> {
    instruction: &'a str,
    operand: Option<&'a str>,
    indirect: bool,
}

impl<'a> ParsedInstruction<'a> {
    fn is_empty(&self) -> bool {
        self.instruction.is_empty()
    }
}

pub fn assemble(messages: &mut Messages, storage: &mut Storage) {

    if storage.source_program.is_empty() {
        messages.error("No source program loaded");
    }

    // Pass one - build symbol table
    let pass_one_result = pass_one(storage);
    messages.combine(pass_one_result);

    if messages.has_errors() {
        return;
    }

    // Pass two - generate binary code
    let pass_two_result = pass_two(storage);
    messages.combine(pass_two_result);

    if !messages.has_errors() {
        messages.info("Assembly completed successfully");
    }
}

// Fast single-pass line parser
fn parse_line_fast(line: &str) -> ParsedInstruction<'_> {
    // Remove comments first
    let line = if let Some(comment_pos) = line.find('/') {
        line[..comment_pos].trim()
    } else {
        line.trim()
    };
    
    if line.is_empty() {
        return ParsedInstruction {
            instruction: "",
            operand: None,
            indirect: false,
        };
    }
    
    // Remove label if present
    let line = if let Some(comma_pos) = line.find(',') {
        line[comma_pos + 1..].trim()
    } else {
        line
    };
    
    if line.is_empty() {
        return ParsedInstruction {
            instruction: "",
            operand: None,
            indirect: false,
        };
    }
    
    // Parse instruction components
    let mut parts = line.split_whitespace();
    let instruction = parts.next().unwrap_or("");
    let operand = parts.next();
    let modifier = parts.next();
    
    let indirect = modifier == Some("i");
    
    ParsedInstruction {
        instruction,
        operand,
        indirect,
    }
}

fn get_mri_table() -> HashMap<&'static str, u16> {
    let mut mri = HashMap::new();
    mri.insert("AND", 0x0000);
    mri.insert("ADD", 0x1000);
    mri.insert("LDA", 0x2000);
    mri.insert("STA", 0x3000);
    mri.insert("BUN", 0x4000);
    mri.insert("BSA", 0x5000);
    mri.insert("ISZ", 0x6000);
    mri
}

fn get_rri_table() -> HashMap<&'static str, u16> {
    let mut rri = HashMap::new();
    rri.insert("CLA", 0x7800);
    rri.insert("CLE", 0x7400);
    rri.insert("CMA", 0x7200);
    rri.insert("CME", 0x7100);
    rri.insert("CIR", 0x7080);
    rri.insert("CIL", 0x7040);
    rri.insert("INC", 0x7020);
    rri.insert("SPA", 0x7010);
    rri.insert("SNA", 0x7008);
    rri.insert("SZA", 0x7004);
    rri.insert("SZE", 0x7002);
    rri.insert("HLT", 0x7001);
    rri
}

fn get_io_table() -> HashMap<&'static str, u16> {
    let mut io = HashMap::new();
    io.insert("INP", 0xF800);
    io.insert("OUT", 0xF400);
    io.insert("SKI", 0xF200);
    io.insert("SKO", 0xF100);
    io.insert("ION", 0xF080);
    io.insert("IOF", 0xF040);
    io
}

pub fn pass_one(storage: &mut Storage) -> Messages {
        let mut messages = Messages::new();
        storage.address_symbol_table.clear();
        let mut origin: u32 = 0;
        
        // First check for ORG on first line
        if !storage.source_program.is_empty() {
            let first_line = &storage.source_program[0];
            if first_line.contains("ORG") {
                let parsed = parse_line_fast(first_line);
                
                if parsed.instruction == "ORG" {
                    if let Some(addr_str) = parsed.operand {
                        if let Ok(_) = addr_str.parse::<u32>() {
                            origin = u32::from_str_radix(addr_str, 16).unwrap_or(0);
                            storage.address_symbol_table.insert("ORG".to_string(), origin);
                            
                            messages.debug(&format!("ORG is at address {}", origin));
                        } else {
                            messages.error("Invalid address for ORG instruction");
                            return messages;
                        }
                    } else {
                        messages.error("Missing address for ORG instruction");
                        return messages;
                    }
                } else {
                    messages.error("Invalid syntax for ORG instruction");
                    return messages;
                }
            } else {
                storage.address_symbol_table.insert("ORG".to_string(), origin);
                messages.debug("No ORG found on first line, setting to 0");
            }
        }
        
        // Check for labels and record locations in address-symbol table
        for i in 0..storage.source_program.len() {
            let line = &storage.source_program[i];
            let parsed = parse_line_fast(line);
            
            if parsed.is_empty() {
                continue;
            }
            
            // Check for END
            if parsed.instruction == "END" {
                messages.debug(&format!("Found END of symbolic program at program line {}", i + 1));
                break;
            }
            
            // Check for label
            if line.contains(',') {
                if let Some(comma_location) = line.find(',') {
                    let label = line[..comma_location].trim().to_string();
                    
                    // Check if invalid
                    if PSEUDO_INSTRUCTIONS.contains(&label.as_str()) {
                        messages.error(&format!("Cannot use invalid label \"{}\"", label));
                        add_detailed_error(&storage.source_program, i as i32, "", "", &mut messages);
                        return messages;
                    } else if storage.address_symbol_table.contains_key(&label) {
                        messages.error(&format!("Label \"{}\" is already used", label));
                        add_detailed_error(&storage.source_program, i as i32, "", "", &mut messages);
                        return messages;
                    } else {
                        let address = i as u32 + origin - 1;
                        storage.address_symbol_table.insert(label.clone(), address);
                        
                        messages.debug(&format!("Found label \"{}\" at program line {}, address {}",
                               label, i + 1, address));
                    }
                }
            }
        }
        
        messages
    }
    
pub fn pass_two(machine: &mut Storage) -> Messages {
    let mut messages = Messages::new();
    machine.assembled_program.clear();

    // Set origin
    let mut origin: u32 = 0;
    if let Some(&origin_val) = machine.address_symbol_table.get("ORG") {
        origin = origin_val;
    } else {
        messages.error("Address symbol table does not contain entry for ORG");
    }

    let mut binary_location = origin as usize;
    messages.debug(&format!("Set binary start location to {}", binary_location));

    let mri_table = get_mri_table();
    let rri_table = get_rri_table();
    let io_table = get_io_table();

    // Check each line and determine type
    for i in 0..machine.source_program.len() {
        let line = &machine.source_program[i];
        
        // Single-pass parsing: extract all components at once
        let parsed = parse_line_fast(line);
        
        if parsed.is_empty() {
            continue;
        }
        
        let mut binary_instruction = String::new();

        // No need for component counting - parse_line_fast handles validation

        // If instruction is a pseudo-instruction
        if PSEUDO_INSTRUCTIONS.contains(&parsed.instruction) {
            match parsed.instruction {
                "ORG" => continue,
                "END" => break,
                "DEC" | "HEX" => {
                    if let Some(operand) = parsed.operand {
                        if let Ok(_) = operand.parse::<i32>() {
                            if parsed.instruction == "DEC" {
                                let dec_operand = operand.parse::<i16>().unwrap_or(0) as u16;
                                binary_instruction = format!("{:04X}", dec_operand);
                            } else if parsed.instruction == "HEX" {
                                let hex_operand = u16::from_str_radix(operand, 16).unwrap_or(0);
                                binary_instruction = format!("{:04X}", hex_operand);
                            }
                            // Ensure assembled_program is large enough
                            while machine.assembled_program.len() <= binary_location {
                                machine.assembled_program.push(String::new());
                            }
                            machine.assembled_program[binary_location] = binary_instruction.clone();
                            messages.debug(&format!("Instruction \"{}\" at program line {} and converted to \"{}\" at binary program location {}",
                                   parsed.instruction, i, binary_instruction, binary_location));
                        } else {
                            messages.error(&format!("Invalid operand \"{}\" for instruction \"{}\"", operand, parsed.instruction));
                        }
                    } else {
                        messages.error("Missing operand");
                    }
                },
                _ => {
                    messages.error(&format!("Invalid pseudoinstruction \"{}\"", parsed.instruction));
                }
            }
        }
        // If instruction is a memory reference instruction
        else if mri_table.contains_key(parsed.instruction) {
            let label = parsed.operand.unwrap_or_else(|| {
                messages.error("Missing operand for memory reference instruction");
                ""
            });

            let label_address = if let Some(&addr) = machine.address_symbol_table.get(label) {
                addr
            } else {
                messages.error(&format!("Unknown label \"{}\"", label));
                0
            };

            let mut hex_binary_instruction = mri_table[parsed.instruction] + label_address as u16;

            // Apply indirect addressing if specified
            if parsed.indirect {
                hex_binary_instruction += 0x8000;
            }

            binary_instruction = format!("{:X}", hex_binary_instruction);
            // Ensure assembled_program is large enough
            while machine.assembled_program.len() <= binary_location {
                machine.assembled_program.push(String::new());
            }
            machine.assembled_program[binary_location] = binary_instruction.clone();
            messages.debug(&format!("Instruction \"{}\" at program line {} and converted to \"{}\" at binary program location {}",
                   parsed.instruction, i, binary_instruction, binary_location));
        }
        // If instruction is a register reference instruction or input/output instruction
        else if rri_table.contains_key(parsed.instruction) || io_table.contains_key(parsed.instruction) {
            if let Some(&opcode) = rri_table.get(parsed.instruction) {
                binary_instruction = format!("{:X}", opcode);
            } else if let Some(&opcode) = io_table.get(parsed.instruction) {
                binary_instruction = format!("{:X}", opcode);
            }

            // Ensure assembled_program is large enough
            while machine.assembled_program.len() <= binary_location {
                machine.assembled_program.push(String::new());
            }
            machine.assembled_program[binary_location] = binary_instruction.clone();
            messages.debug(&format!("Instruction \"{}\" at program line {} and converted to \"{}\" at binary program location {}",
                   parsed.instruction, i, binary_instruction, binary_location));
        }
        // If instruction is not found
        else {
            messages.error(&format!("Unknown instruction \"{}\"", parsed.instruction));
        }

        binary_location += 1;
    }

    // Final check for occurrence of error(s)
    if messages.error_count() > 0 {
        if messages.error_count() == 1 {
            messages.info("Encountered 1 error");
        } else {
            messages.info(&format!("Encountered {} errors", messages.error_count()));
        }
    }

    messages
}

fn add_detailed_error(program: &[String], line_index: i32, error: &str, cause: &str, messages: &mut Messages) {
    if line_index >= 0 {
        let faulty_line = program[line_index as usize].trim();
        messages.error(&format!("Error at line {}: ", line_index));
        messages.error(&format!(" <{}> {}", line_index, faulty_line));
        
        let padding = if !cause.is_empty() && faulty_line.contains(cause) {
            4 + line_index.to_string().len() + faulty_line.find(cause).unwrap_or(0)
        } else {
            4 + line_index.to_string().len()
        };
        
        messages.error(&format!("{}{}", " ".repeat(padding), "^"));
        messages.error(&format!("{}{}", " ".repeat(padding), error));
    } else {
        messages.error("Error in program");
    }
}