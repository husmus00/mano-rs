use crate::message::{Messages, Level};

pub fn print_messages(messages: &Messages) {
    let debug = false;

    for (level, msg) in &messages.entries {
        match level {
            Level::Info => println!("{}", msg),
            Level::Error => println!("ERROR: {}", msg),
            Level::Debug => if debug {
                println!("DEBUG: {}", msg)
            },
        }
    }
}