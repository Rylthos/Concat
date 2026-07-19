use clap::Parser;
use concat::config::config::Config;

use concat::lexer::lexer::Lexer;
use concat::parser::parser::Parser as ConcatParser;
use concat::reducer::reducer::Reducer;
use concat::type_checker;
use concat::type_checker::type_checker::TypeChecker;

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

    println!("AST: {:?}\n", ast);

    let mut reducer = Reducer::init(config.clone(), ast);
    let reduced_ast = match reducer.reduce() {
        Ok(t) => t,
        Err(e) => {
            e.print();
            return;
        }
    };

    println!("Reduced: {:?}\n", reduced_ast);

    let mut type_checker = TypeChecker::init(config.clone(), reduced_ast);
    let typed_tree = match type_checker.type_check() {
        Ok(t) => t,
        Err(e) => {
            e.print();
            return;
        }
    };

    println!("Typed: {:?}\n", typed_tree.main_region);

    // interpret(&parser.instructions, &parser.default_heap)
    //
}
