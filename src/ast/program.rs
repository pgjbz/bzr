use super::statement::Statement;
use std::fmt::Display;

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
    pub errors: Vec<String>,
}

impl Program {
    pub fn new(statements: Vec<Box<dyn Statement>>, errors: Vec<String>) -> Box<Self> {
        Box::new(Self { statements, errors })
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut program = String::new();
        for stmt in self.statements.iter() {
            program.push_str(&stmt.to_string())
        }
        write!(f, "{}", program)
    }
}
