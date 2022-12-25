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

type MjiMap = HashMap<String, MjiMapEntry>;

pub fn list(map: &MjiMap) {}

pub fn find_pr(map: &MjiMap, name: &str) {}

pub fn find(map: &MjiMap, name: &str) {}

fn pre() {}

fn post() {}
