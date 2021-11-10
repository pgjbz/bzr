use std::{fmt::Display, rc::Rc};

pub struct Identifier {
    pub value: Option<Rc<String>>
}

impl Identifier {
    pub fn new(value: Option<Rc<String>>) -> Self {
        Self { value }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value.as_ref().unwrap())
    }
}