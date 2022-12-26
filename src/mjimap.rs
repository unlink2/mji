use crate::{Error, CFG};
use console::{style, Emoji, Term};
use std::{collections::HashMap, fmt::Display};

#[derive(Clone)]
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

pub fn list(map: &MjiMap) {
    let term = Term::stdout();
    let name_width = 0;
    let val_width = 10;
    map.iter().for_each(|(_key, c)| {
        println!(
            "{:<val_width$} | {:<name_width$} {}",
            Emoji(&c.value, ""),
            style(&c.name).bold(),
            style(&c.desc)
        )
    });
}

pub fn find_or(map: &MjiMap, inputs: &[&str]) -> Result<(), Error> {
    let mut should_find = true;
    for input in inputs {
        if should_find {
            let mji = find(map, input)?;
            pre();
            print!("{}", mji.value);
            should_find = false;
        } else if input == &"-" {
            should_find = true;
            post();
        } else {
            print!(" {}", input);
        }
    }
    post();
    Ok(())
}

pub fn find(map: &MjiMap, input: &str) -> Result<MjiMapEntry, Error> {
    if let Some(mji) = map.get(input) {
        Ok(mji.clone())
    } else {
        Err(Error::NotFound)
    }
}

fn pre() {
    print!("{}", CFG.pre);
}

fn post() {
    print!("{}", CFG.post);
}
