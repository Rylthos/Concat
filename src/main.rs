use clap::Parser;
use concat::codegen::codegen::CodeGen;
use concat::config::config::Config;

use concat::ir::ir::IR;
use concat::lexer::lexer::Lexer;
use concat::parser::parser::Parser as ConcatParser;
use concat::reducer::reducer::Reducer;
use concat::type_checker;
use concat::type_checker::type_checker::TypeChecker;
use concat::vm::vm::VM;

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

    let mut type_checker = TypeChecker::init(config.clone(), reduced_ast);
    let typed_data = match type_checker.type_check() {
        Ok(t) => t,
        Err(e) => {
            e.print();
            return;
        }
    };

    let mut ir = IR::init(config.clone(), typed_data);
    let ir_instructions = match ir.generate_ir_instructions() {
        Ok(instr) => instr,
        Err(e) => {
            e.print();
            return;
        }
    };

    let mut codegen = CodeGen::init(config.clone(), ir_instructions);
    let instructions = match codegen.generate_vm() {
        Ok(instr) => instr,
        Err(e) => {
            e.print();
            return;
        }
    };

    let mut vm = VM::init(config.clone(), instructions);
    vm.interpret();
}
