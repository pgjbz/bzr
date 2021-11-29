use std::fmt::Display;

pub trait Node: Display {
    fn literal(&self) -> Box<dyn Display>;
}
