use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    pub path: std::path::PathBuf,

    #[arg(short, long, default_value_t = true)]
    pub expr_print: bool,

    #[arg(short, long, default_value_t = true)]
    pub token_print: bool,
}

impl Config {
    pub fn blank() -> Config {
        Config {
            path: "".into(),
            expr_print: false,
            token_print: false,
        }
    }
}
