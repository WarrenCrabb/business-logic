use std::{cell::RefCell, collections::HashMap, hash::Hash, hash::Hasher, rc::Rc};

use super::Object;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Env>,
}

pub type Env = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new() -> Env {
        Rc::new(RefCell::new(Self::default()))
    }

    pub fn new_enclosed_environment(outer: &Env) -> Env {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: Some(Rc::clone(outer)),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer {
                Some(env) => env.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}

impl Hash for Environment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in self.store.iter() {
            key.hash(state);
            value.hash(state);
        }
    }
}
