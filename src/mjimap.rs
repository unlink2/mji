use std::collections::HashMap;

pub struct MjiMapEntry {
    pub name: String,
    pub value: String,
    pub desc: String,
}

impl MjiMapEntry {
    pub fn new(name: &str, value: &str, desc: &str) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            desc: desc.into(),
        }
    }
}

pub type MjiMap = HashMap<String, MjiMapEntry>;

pub fn list(map: &MjiMap) {}

pub fn find_or<T: AsRef<str>>(map: &MjiMap, inputs: &[T]) {}

pub fn find<T: AsRef<str>>(map: &MjiMap, inputs: &[T]) {}

fn pre() {}

fn post() {}
