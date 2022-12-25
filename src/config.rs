use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CFG: Config = Config::new();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, default_value_t = false)]
    pub list: bool,
    #[arg(long, default_value_t = String::from(""))]
    pub pre: String,
    #[arg(long, default_value_t = String::from("\n"))]
    pub post: String,

    pub inputs: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            list: false,
            pre: "".into(),
            post: "".into(),
            inputs: vec![],
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }
}
