use std::collections::HashMap;

use super::token::Object;

#[derive(Debug)]
pub struct VM {
    pub values: HashMap<String, Object>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn retrieve(&self, name: String) -> &Object {
        if let Some(object) = self.values.get(&name) {
            object
        } else {
            panic!("undefined variable")
        }
    }
}
