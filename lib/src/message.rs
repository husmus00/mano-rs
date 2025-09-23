

#[derive(Debug, Clone)]
pub enum Level {
    Info,
    Error,
    Debug,
}

#[derive(Debug, Clone, Default)]
pub struct Messages {
    pub entries: Vec<(Level, String)>,
    pub status: u16
}

impl Messages {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn info(&mut self, msg: &str) {
        self.entries.push((Level::Info, msg.to_string()));
    }
    
    pub fn error(&mut self, msg: &str) {
        self.entries.push((Level::Error, msg.to_string()));
    }

    pub fn debug(&mut self, msg: &str) {
        self.entries.push((Level::Debug, msg.to_string()));
    }
    
    pub fn has_errors(&self) -> bool {
        self.entries.iter().any(|(level, _)| matches!(level, Level::Error))
    }
    
    pub fn error_count(&self) -> usize {
        self.entries.iter().filter(|(level, _)| matches!(level, Level::Error)).count()
    }
    
    pub fn combine(&mut self, other: Messages) {
        self.entries.extend(other.entries);
    }
}