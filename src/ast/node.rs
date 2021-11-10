use std::fmt::Display;

pub trait Node {
    fn literal(&self) -> Box<dyn Display>;
}
