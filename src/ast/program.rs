use super::statement::Statement;

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
    pub erros: Vec<String>
}

impl Program {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Box<Self> {
        Box::new(Self { statements, erros: vec![]})
    }
}