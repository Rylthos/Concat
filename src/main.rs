use concat::input::read_file;
use concat::interpreter::interpreter::interpret;
use concat::lexer::lexer::lex_string;
use concat::parser::parser::parse_tokens;

fn main() {
    let input = read_file();
    let tokens = lex_string(&input);
    let expr = parse_tokens(&tokens);
    interpret(&expr)
}
