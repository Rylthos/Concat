use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    pub path: std::path::PathBuf,

    #[arg(short, long = "expr", default_value_t = false)]
    pub expr_print: bool,

    #[arg(long = "lexer", default_value_t = false)]
    pub lexer_print: bool,

    #[arg(long = "parser", default_value_t = false)]
    pub parser_print: bool,

    #[arg(long = "reduce", default_value_t = false)]
    pub reduce_print: bool,

    #[arg(long = "type", default_value_t = false)]
    pub type_print: bool,

    #[arg(long = "ir", default_value_t = false)]
    pub ir_print: bool,

    #[arg(long = "codegen", default_value_t = false)]
    pub codegen_print: bool,
}

impl Config {
    pub fn blank() -> Config {
        Config {
            path: "".into(),
            expr_print: false,
            lexer_print: false,
            parser_print: false,
            reduce_print: false,
            type_print: false,
            ir_print: false,
            codegen_print: false,
        }
    }
}
