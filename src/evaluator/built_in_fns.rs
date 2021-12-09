use std::rc::Rc;

use crate::object::{integer::Integer, string::Str, Object};

pub fn len(args: &[Rc<dyn Object>]) -> Rc<dyn Object> {
    let mut len = 0;
    if args.len() == 1 {
        if let Some(string) = args[0].as_any().downcast_ref::<Str>() {
            len = string.val.len()
        } else {
            len = args.len()
        }
    }
    Rc::new(Integer::new(len as i64))
}

pub fn print(args: &[Rc<dyn Object>]) -> Rc<dyn Object> {
    let mut buffer = String::new();
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    println!("{}", buffer);
    let string = Str::new(buffer);
    Rc::new(string)
}