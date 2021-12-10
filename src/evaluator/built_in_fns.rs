use std::rc::Rc;

use crate::object::{array::Array, error::Error, integer::Integer, string::Str, Object};

pub fn len(args: &[Rc<dyn Object>]) -> Rc<dyn Object> {
    let mut len = 0;
    if args.len() == 1 {
        if let Some(string) = args[0].as_any().downcast_ref::<Str>() {
            len = string.val.len()
        } else if let Some(arr) = args[0].as_any().downcast_ref::<Array>() {
            len = arr.elements.borrow_mut().len()
        }
        Rc::new(Integer::new(len as i64))
    } else {
        Rc::new(Error::new("wrong number of arguments".to_string()))
    }
}

pub fn append(args: &[Rc<dyn Object>]) -> Rc<dyn Object> {
    if args.len() < 2 {
        Rc::new(Error::new("wrong number of arguments".to_string()))
    } else if let Some(arr) = args[0].as_any().downcast_ref::<Array>() {
        for element in args[1..].iter() {
            arr.elements.borrow_mut().push(Rc::clone(element))
        }
        Rc::clone(&args[0])
    } else if let Some(str) = args[0].as_any().downcast_ref::<Str>() {
        let mut buffer = String::new();
        buffer.push_str(&str.val);
        for arg in args[1..].iter() {
            buffer.push_str(&format!("{}", arg));
        }
        Rc::new(Str::new(buffer))
    } else {
        Rc::new(Error::new("first argument must be array".to_string()))
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
    if args.is_empty() {
        return Rc::new(Error::new("must have more than 0 arguments".to_string()));
    }
    let mut buffer = String::new();
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    Rc::new(Str::new(buffer))
}

pub fn to_int(args: &[Rc<dyn Object>]) -> Rc<dyn Object> {
    if args.len() != 1 {
        return Rc::new(Error::new("invalid number of arguments".to_string()));
    }
    let buffer = args[0].to_string();
    match buffer.trim().parse() {
        Ok(val) => Rc::new(Integer::new(val)),
        Err(_) => Rc::new(Error::new(format!(
            "invalid value to parse int: {}",
            args[0]
        ))),
    }
}
