use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CFG: Config = Config::new();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    list: bool,
    #[arg(long)]
    pre: String,
    #[arg(long)]
    post: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            list: false,
            pre: "".into(),
            post: "".into(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }
}
