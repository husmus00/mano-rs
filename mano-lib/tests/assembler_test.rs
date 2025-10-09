use mano_lib::{assembler, storage, message::Messages};

/// Basic addition program: loads A, adds B, stores in C, then halts.
/// Tests ORG, labels, MRI instructions, RRI instruction, DEC pseudo-instructions, and END.
const BASIC_ADDITION_PROGRAM: &[&str] = &[
    "   ORG 0    /Origin of program is location 0",
    "   LDA A    /Load operand from location A", 
    "   ADD B    /Add operand from location B",
    "   STA C    /Store sum in location C",
    "   HLT      /Halt computer",
    "A, DEC 83   /Decimal operand",
    "B, DEC -23  /Decimal operand", 
    "C, DEC 0    /Sum stored in location C",
    "   END      /End of symbolic program",
];

/// Expected machine code output for basic addition program
const BASIC_ADDITION_EXPECTED: &[&str] = &[
    "2004",  // LDA A (Load from address 4)
    "1005",  // ADD B (Add from address 5)
    "3006",  // STA C (Store to address 6) 
    "7001",  // HLT (Halt)
    "0053",  // DEC 83 (Decimal 83 = 0x53)
    "FFE9",  // DEC -23 (Two's complement of -23)
    "0000",  // DEC 0
];

/// Program with invalid instruction to test error handling
const INVALID_INSTRUCTION_PROGRAM: &[&str] = &[
    "   ORG 0    /Origin of program is location 0",
    "   LDA A    /Valid instruction",
    "   BADOP B  /Invalid instruction - should cause error", 
    "   HLT      /Valid instruction",
    "A, DEC 5    /Decimal operand",
    "B, DEC 10   /Decimal operand",
    "   END      /End of symbolic program",
];

#[test]
fn test_basic_addition_program_assembly() {
    let mut storage = storage::Storage::new();
    let mut messages = Messages::new();

    let program_lines: Vec<String> = BASIC_ADDITION_PROGRAM.iter().map(|s| s.to_string()).collect();
    storage.load_program(program_lines, &mut messages);

    assembler::assemble(&mut storage, &mut messages);
    assert!(!messages.has_errors(), "Assembly should complete without errors");

    let assembled_len = storage.assembled_program.len();
    let expected_len = BASIC_ADDITION_EXPECTED.len();

    assert_eq!(assembled_len, expected_len, "Assembly should produce {} instructions", expected_len);

    for (i, expected) in BASIC_ADDITION_EXPECTED.iter().enumerate() {
        let actual = &storage.assembled_program[i];
        assert!(!actual.trim().is_empty(), "Instruction {} should not be empty", i);
        assert_eq!(actual, expected, "Instruction {} should match expected output", i);
    }
}

#[test]
fn test_invalid_instruction_error() {
    let mut storage = storage::Storage::new();
    let mut messages = Messages::new();

    let program_lines: Vec<String> = INVALID_INSTRUCTION_PROGRAM.iter().map(|s| s.to_string()).collect();
    storage.load_program(program_lines, &mut messages);

    assembler::assemble(&mut storage, &mut messages);
    assert!(messages.has_errors(), "Assembly should fail with errors for invalid instruction");

    let has_badop_error = messages.entries.iter()
        .any(|(level, msg)| {
            matches!(level, mano_lib::message::Level::Error)
            && msg.contains("Unknown instruction")
            && msg.contains("BADOP")
        });

    assert!(has_badop_error, "Should have error about unknown instruction 'BADOP'");
}