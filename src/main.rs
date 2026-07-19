use clap::Parser;
use concat::config::config::Config;

use concat::lexer::lexer::Lexer;
use concat::parser::parser::Parser as ConcatParser;
use concat::reducer::reducer::Reducer;

fn main() {
    let config = Config::parse();

    let mut lexer = Lexer::init(config.clone(), config.path.clone());
    match lexer.lex_input() {
        Ok(_) => (),
        Err(e) => {
            e.print();
            return;
        }
    }

    let mut parser = ConcatParser::init(config.clone(), lexer.tokens);
    let ast = match parser.parse() {
        Ok(t) => t,
        Err(e) => {
            e.print();
            return;
        }
    };

    let mut reducer = Reducer::init(config.clone(), ast);
    let reduced_ast = match reducer.reduce() {
        Ok(t) => t,
        Err(e) => {
            e.print();
            return;
        }
    };

    // interpret(&parser.instructions, &parser.default_heap)
    //
}
