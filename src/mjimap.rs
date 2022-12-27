use crate::{config::HeaderMode, Error, CFG};
use console::{style, Emoji};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::Write, process::Command};

const MJI_WRAPPER: char = ':';

#[derive(Clone, Deserialize, Serialize)]
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

    pub fn hint_name(&self) -> String {
        format!("{}{}{}", MJI_WRAPPER, self.name, MJI_WRAPPER)
    }

    pub fn is_mji(input: &str) -> bool {
        input.trim().starts_with(MJI_WRAPPER) && input.trim().ends_with(MJI_WRAPPER)
    }
}

pub type MjiMap = HashMap<String, MjiMapEntry>;

pub fn mji_map_from_file(path: &str) -> anyhow::Result<MjiMap> {
    let data = fs::read_to_string(path)?;
    Ok(toml::from_str(&data)?)
}

pub fn mji_map_join(map: &mut MjiMap, with: &MjiMap) {
    with.iter().for_each(|(key, value)| {
        map.insert(key.to_owned(), value.clone());
    });
}

pub fn list(f: &mut dyn Write, map: &MjiMap) {
    let name_width = 0;
    let val_width = 10;
    map.iter().for_each(|(_key, c)| {
        writeln!(
            f,
            "{:<val_width$} | {:<name_width$} {}",
            Emoji(&c.value, ""),
            style(&c.name).bold(),
            style(&c.desc)
        )
        .unwrap()
    });
}

pub fn commit(buffer: &[u8]) -> Result<(), Error> {
    let mut parsed_cmd = CFG
        .read()
        .unwrap()
        .commit_cmd
        .split_whitespace()
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();

    parsed_cmd.push(String::from_utf8(buffer.to_vec()).unwrap_or("".to_string()));

    let output = Command::new(parsed_cmd.first().unwrap_or(&"".to_string()))
        .args(&parsed_cmd[1..])
        .args(&CFG.read().unwrap().escaped)
        .status();

    if output.is_ok() {
        Ok(())
    } else {
        Err(Error::CommandFailed)
    }
}

pub fn header(f: &mut dyn Write) -> Result<(), Error> {
    match CFG.read().unwrap().header_mode {
        HeaderMode::Static => write!(
            f,
            "{}{}{}",
            CFG.read().unwrap().header_pre,
            CFG.read().unwrap().header_cmd,
            CFG.read().unwrap().header_post
        )
        .unwrap(),
        HeaderMode::Command => {
            let parsed_cmd = CFG
                .read()
                .unwrap()
                .header_cmd
                .split_whitespace()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();

            let output = Command::new(parsed_cmd.first().unwrap_or(&"".to_string()))
                .args(&parsed_cmd[1..])
                .output();
            if let Ok(output) = output {
                write!(
                    f,
                    "{}{}{}",
                    CFG.read().unwrap().header_pre,
                    String::from_utf8(output.stdout)
                        .unwrap_or("<output is not utf8 encoded>".into()),
                    CFG.read().unwrap().header_post
                )
                .unwrap();
            } else {
                return Err(Error::CommandFailed);
            }
        }
        HeaderMode::NoHeader => {}
    }

    Ok(())
}

pub fn find_or(f: &mut dyn Write, map: &MjiMap, inputs: &[&str]) -> Result<(), Error> {
    header(f)?;

    let mut first = true;

    for input in inputs {
        if MjiMapEntry::is_mji(input) {
            let mji = find(map, input)?;
            pre(f);
            if !first {
                write!(f, " ").unwrap();
            }
            write!(f, "{}", mji.value).unwrap();
            first = false;
        } else if input == &"-" {
            first = true;
            post(f);
        } else {
            write!(f, " {input}").unwrap();
            first = false;
        }
    }
    post(f);
    Ok(())
}

pub fn find(map: &MjiMap, input: &str) -> Result<MjiMapEntry, Error> {
    let input = input.trim();
    let input = input.strip_prefix(MJI_WRAPPER).unwrap_or(input);
    let input = input.strip_suffix(MJI_WRAPPER).unwrap_or(input);

    if let Some(mji) = map.get(input) {
        Ok(mji.clone())
    } else {
        Err(Error::NotFound)
    }
}

fn pre(f: &mut dyn Write) {
    write!(f, "{}", CFG.read().unwrap().pre).unwrap();
}

fn post(f: &mut dyn Write) {
    write!(f, "{}", CFG.read().unwrap().post).unwrap();
}
