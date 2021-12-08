use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Object;

pub struct Environment {
    pub store: HashMap<String, Rc<dyn Object>>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(outer: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            store: HashMap::new(),
            outer,
        }
    }

    pub fn set(&mut self, name: String, obj: Rc<dyn Object>) {
        self.store.insert(name, obj);
    }

    pub fn get(&mut self, name: String) -> Option<Rc<dyn Object>> {
        if let Some(obj) = self.store.get(&name) {
            Some(Rc::clone(obj))
        } else if let Some(ref outer) = self.outer {
            for (key, val) in outer.borrow_mut().store.iter() {
                eprintln!("{} = {}", key, val);
            }
            if let Some(obj) = outer.borrow_mut().get(name) {
                Some(Rc::clone(&obj))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(None)
    }
}
