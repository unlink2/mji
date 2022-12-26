use crate::{config::HeaderMode, Error, CFG};
use console::{style, Emoji};
use std::{collections::HashMap, process::Command};

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

pub fn header() -> Result<(), Error> {
    match CFG.header_mode {
        HeaderMode::Static => println!("{}", CFG.header_cmd),
        HeaderMode::Command => {
            let output = Command::new(CFG.header_cmd.to_owned()).output();
            if let Ok(output) = output {
                println!(
                    "{}",
                    String::from_utf8(output.stdout).unwrap_or("..".into())
                );
            } else {
                return Err(Error::CommandFailed);
            }
        }
    }

    Ok(())
}

pub fn find_or(map: &MjiMap, inputs: &[&str]) -> Result<(), Error> {
    header()?;
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
