use std::fmt::Display;

use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CFG: Config = Config::new();
}

const HEADER_COMMAND_DEFAULT: &str = "git rev-parse --abbrev-ref HEAD";

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum HeaderMode {
    NoHeader,
    Command,
    Static,
}

impl Default for HeaderMode {
    fn default() -> Self {
        Self::Command
    }
}

impl Display for HeaderMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Command => write!(f, "command"),
            Self::Static => write!(f, "static"),
            Self::NoHeader => write!(f, "no-header"),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, default_value_t = false)]
    pub list: bool,
    #[arg(long, default_value_t = String::from(""))]
    pub pre: String,
    #[arg(long, default_value_t = String::from("\n"))]
    pub post: String,

    #[arg(long, default_value_t = HeaderMode::Command)]
    pub header_mode: HeaderMode,
    #[arg(long, default_value_t = String::from(HEADER_COMMAND_DEFAULT))]
    pub header_cmd: String,

    #[arg(long, default_value_t = String::from("📫 "))]
    pub header_pre: String,
    #[arg(long, default_value_t = String::from("\n"))]
    pub header_post: String,

    pub inputs: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            list: false,
            pre: "".into(),
            post: "".into(),
            inputs: vec![],
            header_mode: HeaderMode::Command,
            header_cmd: "".into(),
            header_pre: "".into(),
            header_post: "".into(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }
}
