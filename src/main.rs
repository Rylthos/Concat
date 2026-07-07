use clap::Parser;
use concat::config::config::Config;

use concat::error::types::print_error;
use concat::interpreter::interpreter::interpret;
use concat::lexer::lexer::Lexer;
use concat::parser::parser::Parser as ConcatParser;

fn main() {
    let config = Config::parse();

    let mut lexer = Lexer::init(config.clone(), config.path.clone());
    match lexer.lex_input() {
        Ok(_) => (),
        Err(e) => {
            print_error(e);
            return;
        }
    }

    let mut parser = ConcatParser::init(config.clone(), lexer.tokens);
    match parser.parse() {
        Ok(_) => (),
        Err(e) => {
            print_error(e);
            return;
        }
    }

    interpret(&parser.instructions, &parser.default_heap)
}
