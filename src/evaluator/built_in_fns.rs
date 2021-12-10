use std::rc::Rc;

use crate::object::{integer::Integer, string::Str, Object, error::Error, array::Array};

pub fn len(args: &[Rc<dyn Object>]) -> Rc<dyn Object> {
    let mut len = 0;
    if args.len() == 1 {
        if let Some(string) = args[0].as_any().downcast_ref::<Str>() {
            len = string.val.len()
        } else if let Some(arr) = args[0].as_any().downcast_ref::<Array>() {
            len = arr.elements.len()
        }
        Rc::new(Integer::new(len as i64))
    } else {
        Rc::new(Error::new("wrong number of arguments".to_string()))
    }
}

pub fn puts(args: &[Rc<dyn Object>]) -> Rc<dyn Object> {
    let mut buffer = String::new();
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    print!("{}", buffer);
    let string = Str::new(buffer);
    Rc::new(string)
}

pub fn putsln(args: &[Rc<dyn Object>]) -> Rc<dyn Object> {
    let mut buffer = String::new();
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    println!("{}", buffer);
    let string = Str::new(buffer);
    Rc::new(string)
}

pub fn eputs(args: &[Rc<dyn Object>]) -> Rc<dyn Object> {
    let mut buffer = String::new();
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    eprint!("{}", buffer);
    let string = Str::new(buffer);
    Rc::new(string)
}

pub fn eputsln(args: &[Rc<dyn Object>]) -> Rc<dyn Object> {
    let mut buffer = String::new();
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    eprintln!("{}", buffer);
    let string = Str::new(buffer);
    Rc::new(string)
}

pub fn to_str(args: &[Rc<dyn Object>]) -> Rc<dyn Object> {
    let mut buffer = String::new();
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    Rc::new(Str::new(buffer))
}
