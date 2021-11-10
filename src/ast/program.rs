use super::statement::Statement;

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
    pub errors: Vec<String>,
}

impl Program {
    pub fn new(statements: Vec<Box<dyn Statement>>, errors: Vec<String>) -> Box<Self> {
        Box::new(Self { statements, errors })
    }
}
