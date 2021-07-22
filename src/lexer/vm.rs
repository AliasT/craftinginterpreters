use std::collections::HashMap;

use super::token::{Object, UnionObject};

#[derive(Debug)]
pub struct VM<'a> {
    pub values: HashMap<String, UnionObject<'a>>,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: UnionObject<'a>) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: UnionObject<'a>) {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
        }
        // TODO: 返回赋值
        // *value
    }

    pub fn retrieve(&self, name: String) -> &UnionObject<'a> {
        if let Some(object) = self.values.get(&name) {
            object
        } else {
            panic!("undefined variable")
        }
    }
}
