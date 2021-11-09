use super::statement::Statement;

pub struct Program<'a> {
    pub statements: Vec<Box<dyn Statement<'a>>>,
    pub erros: Vec<String>
}

impl<'a> Program<'a> {
    pub fn new(statements: Vec<Box<dyn Statement<'a>>>) -> Box<Self> {
        Box::new(Self { statements, erros: vec![]})
    }
}
