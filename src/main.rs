use clap::Parser;
use concat::config::config::Config;

use concat::input::read_file_path;
use concat::interpreter::interpreter::interpret;
use concat::lexer::lexer::Lexer;
use concat::parser::parser::parse_tokens;

fn main() {
    let config = Config::parse();

    let input = read_file_path(&config.path);

    let mut lexer = Lexer::init(config, input);
    match lexer.lex_input() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }

    let expr = parse_tokens(&lexer.tokens);
    interpret(&expr)
}
