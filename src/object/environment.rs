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
        if self.exists_in_outer(&name) {
            if let Some(ref out) = self.outer {
                out.borrow_mut().set(name, obj);
            }
        } else {
            self.store.insert(name, obj);
        }
    }

    pub fn get(&mut self, name: String) -> Option<Rc<dyn Object>> {
        if let Some(obj) = self.store.get(&name) {
            Some(Rc::clone(obj))
        } else if let Some(ref outer) = self.outer {
            if let Some(obj) = outer.borrow_mut().get(name) {
                Some(Rc::clone(&obj))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn exists_in_outer(&mut self, name: &String) -> bool{
        if let Some(ref outer) = self.outer {
            outer.borrow_mut().get(name.clone()).is_some()
        } else {
            false
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(None)
    }
}
