use std::fmt::Debug;

use crate::engine::tokens::Token;

pub enum EndiumError {
    // Critical
    CriticalFileNotFound(String),
    AssignmentToConstantVariable(String),
    NotDefinedError(String),

    // Error
    UndefinedError(String),

    // Warnings
    FileNotFound(String),
}

impl std::fmt::Display for EndiumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndiumError::CriticalFileNotFound(file) => {
                self.panic(format!("File {} not found", file))
            }
            EndiumError::FileNotFound(file) => self.error(format!("File {} not found", file)),
            _ => {}
        }

        Ok(())
    }
}

impl EndiumError {
    pub fn panic<T: Debug>(&self, message: T) {
        panic!("[Endium Critical Error]: {:#?}", message);
    }

    pub fn error<T: Debug>(&self, message: T) {
        eprintln!("[Endium Error]: {:#?}", message);
    }

    pub fn warning<T: Debug>(&self, message: T) {
        println!("[Endium Warning]: {:#?}", message);
    }
}
