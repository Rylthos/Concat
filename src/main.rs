use clap::Parser;
use concat::config::config::Config;

use concat::input::read_file_path;
use concat::interpreter::interpreter::interpret;
use concat::lexer::lexer::Lexer;
use concat::parser::parser::Parser as ConcatParser;

fn main() {
    let config = Config::parse();

    let input = read_file_path(&config.path);

    let mut lexer = Lexer::init(config.clone(), input);
    match lexer.lex_input() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }

    let mut parser = ConcatParser::init(config.clone(), lexer.tokens);
    match parser.parse() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }

    interpret(&parser.instructions)
}
