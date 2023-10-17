use crate::val::Val;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Env<'parent> {
    bindings: HashMap<String, Val>,
    parent: Option<&'parent Self>,
}

impl<'parent> Env<'parent> {
    pub(crate) fn store_binding(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }

    pub(crate) fn get_binding(&self, name: &str) -> Result<Val, String> {
        self.get_binding_no_error(name)
            .ok_or_else(|| format!("Binding does not exist: {name}"))
    }

    fn get_binding_no_error(&self, name: &str) -> Option<Val> {
        self.bindings.get(name).cloned().or_else(|| {
            self.parent
                .and_then(|parent| parent.get_binding_no_error(name))
        })
    }

    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
        }
    }
}
