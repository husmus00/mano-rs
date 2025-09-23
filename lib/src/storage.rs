use std::collections::HashMap;
use crate::message::Messages;

pub struct Storage {
    pub source_program: Vec<String>,
    pub assembled_program: Vec<String>,
    pub address_symbol_table: HashMap<String, u32>
}

 impl Storage {
     pub fn new() -> Self {
         Self {
             source_program: Vec::new(),
             assembled_program: Vec::new(),
             address_symbol_table: HashMap::new(),
         }
     }

     pub fn load_program(&mut self, program: Vec<String>) -> Messages {
         self.source_program = program;
         self.assembled_program.clear();
         self.address_symbol_table.clear();

         let mut messages = Messages::new();
         messages.info("Program loaded successfully");
         messages
     }

     pub fn clear(&mut self) -> Messages {
         self.source_program.clear();
         self.assembled_program.clear();
         self.address_symbol_table.clear();

         let mut messages = Messages::new();
         messages.info("Program cleared");
         messages
     }
 }