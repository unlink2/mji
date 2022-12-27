use std::fmt::Display;

use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub static ref CFG: RwLock<Config> = RwLock::new(Config::new());
}

fn header_default_command() -> String {
    std::env::var("MJI_HEADER_COMMAND").unwrap_or("git rev-parse --abbrev-ref HEAD".into())
}

fn commit_default_command() -> String {
    std::env::var("MJI_COMMIT_COMMAND").unwrap_or("git commit -e -am ".into())
}

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

#[derive(Parser, Clone)]
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
    #[arg(long, default_value_t = header_default_command())]
    pub header_cmd: String,

    // TODO talk about the icon for a commit header...
    //      reasoning here: a commit is signaled by leaving a
    //      commit message...
    #[arg(long, default_value_t = String::from("📫 "))]
    pub header_pre: String,
    #[arg(long, default_value_t = String::from("\n"))]
    pub header_post: String,

    #[arg(long, default_value_t = false)]
    pub commit: bool,

    #[arg(long, default_value_t = commit_default_command())]
    pub commit_cmd: String,

    pub inputs: Vec<String>,

    #[arg(last = true)]
    pub escaped: Vec<String>,
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
            commit: false,
            commit_cmd: "".into(),
            escaped: vec![],
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }
}
