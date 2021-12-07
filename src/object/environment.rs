use std::{collections::HashMap, rc::Rc};

use super::Object;

pub struct Environment {
    pub store: HashMap<String, Rc<dyn Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, obj: Rc<dyn Object>) {
        self.store.insert(name, obj);
    }

    pub fn get(&mut self, name: String) -> Option<Rc<dyn Object>> {
        if let Some(obj) = self.store.get(&name) {
            Some(Rc::clone(obj))
        } else {
            None
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
