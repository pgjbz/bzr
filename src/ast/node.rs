pub trait Node<'a> {
    fn literal(&self) -> &'a str;
}
