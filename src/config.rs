use std::fmt::Display;

use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use std::sync::RwLock;

use crate::{
    gitmoji::GITMOJI,
    mjimap::{mji_map_from_file, mji_map_join, MjiMap},
    print_error_and_exit,
};

lazy_static! {
    pub static ref CFG: RwLock<Config> = RwLock::new(Config::new());
}

fn header_default_command() -> String {
    std::env::var("MJI_HEADER_COMMAND").unwrap_or("git rev-parse --abbrev-ref HEAD".into())
}

fn commit_default_command() -> String {
    std::env::var("MJI_COMMIT_COMMAND").unwrap_or("git commit -e -am ".into())
}

fn get_default_map_path() -> String {
    let path = dirs::preference_dir()
        .unwrap_or("./".into())
        .join("mji.toml");

    if path.exists() {
        path.to_str().unwrap_or("-").to_owned()
    } else {
        // FIXME do we need a sentinel value to make clap happy?
        "-".to_owned()
    }
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

#[derive(Parser, Clone, Default)]
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

    #[arg(long, default_value_t = false)]
    no_join_default_map: bool,

    #[arg(long, default_value_t = get_default_map_path())]
    custom_map_path: String,

    #[arg(long, short)]
    pub out: Option<String>,

    #[arg(long, short, default_value_t = false)]
    pub quiet: bool,

    #[arg(long, default_value_t = false)]
    pub hook: bool,

    #[arg(long, default_value_t = false)]
    pub no_mji_find_error: bool,

    #[arg(long, short)]
    pub input: Option<String>,

    pub inputs: Vec<String>,

    #[arg(last = true)]
    pub escaped: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut cfg = Self::parse();

        if let Some(input) = &cfg.input.clone() {
            // FIXME maybe we should not hard crash here...
            print_error_and_exit!(cfg.input_from_file(&input));
        }

        cfg
    }

    pub fn mji_map(&self) -> anyhow::Result<MjiMap> {
        Ok(if self.custom_map_path != "-" {
            let loaded = mji_map_from_file(&self.custom_map_path)?;

            if self.no_join_default_map {
                loaded
            } else {
                let mut result = GITMOJI.clone();
                mji_map_join(&mut result, &loaded);
                result
            }
        } else {
            GITMOJI.clone()
        })
    }

    pub fn input_from_file(&mut self, path: &str) -> anyhow::Result<()> {
        self.inputs.push(std::fs::read_to_string(path)?);
        Ok(())
    }
}
