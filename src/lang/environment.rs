use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::token::UnionObject;

#[derive(Debug)]
pub struct Environment<'a> {
    pub values: HashMap<String, Rc<UnionObject<'a>>>,
    pub enclosing: Option<Rc<RefCell<Environment<'a>>>>,
}

impl<'a> Environment<'a> {
    pub fn new<T: Into<Option<Rc<RefCell<Environment<'a>>>>>>(enclosing: T) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: enclosing.into(),
        }
    }

    pub fn define(&mut self, name: String, value: Rc<UnionObject<'a>>) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Rc<UnionObject<'a>>) {
        if let Some(e) = &self.enclosing {
            return e.borrow_mut().assign(name, value);
        }

        if self.values.contains_key(&name) {
            self.values.insert(name, value);
        }
        // TODO: 返回赋值
        // *value
    }

    pub fn retrieve(&self, name: String) -> Rc<UnionObject<'a>> {
        if let Some(object) = self.values.get(&name) {
            object.clone()
        } else if let Some(e) = &self.enclosing {
            // 作用域查找
            return e.borrow_mut().retrieve(name);
        } else {
            panic!("undefined variable")
        }
    }
}
