#[cfg(test)]
mod tests {
    use super::super::lexer::Lexer;
    use crate::config::config::Config;
    use crate::lexer::tokens::{Token, TokenType};

    use std::path::PathBuf;

    fn test_input(input: &str, expected_output: &Vec<Token>) {
        let mut lexer = Lexer::init(Config::blank(), PathBuf::new());
        let result = lexer.scan_string(&PathBuf::new(), input);
        match result {
            Ok(t) => assert_eq!(format!("{:?}", expected_output), format!("{:?}", t)),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[test]
    fn lex_single_characters() {
        let input = "+ - {} *\t/ []";
        let output = vec![
            Token::new(TokenType::Add, 1, 1, "", "+"),
            Token::new(TokenType::Subtract, 1, 3, "", "-"),
            Token::new(TokenType::LeftBrace, 1, 5, "", "{"),
            Token::new(TokenType::RightBrace, 1, 6, "", "}"),
            Token::new(TokenType::Asterisk, 1, 8, "", "*"),
            Token::new(TokenType::Divide, 1, 10, "", "/"),
            Token::new(TokenType::LeftSqBracket, 1, 12, "", "["),
            Token::new(TokenType::RightSqBracket, 1, 13, "", "]"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_keywords() {
        let input = "i32 void bool print true false char '\\0' \"Hello, World!\"";
        let output = vec![
            Token::new(TokenType::I32, 1, 1, "", "i32"),
            Token::new(TokenType::Void, 1, 5, "", "void"),
            Token::new(TokenType::Bool, 1, 10, "", "bool"),
            Token::new(TokenType::Print, 1, 15, "", "print"),
            Token::new(TokenType::BoolValue(true), 1, 21, "", "true"),
            Token::new(TokenType::BoolValue(false), 1, 26, "", "false"),
            Token::new(TokenType::Char, 1, 32, "", "char"),
            Token::new(TokenType::CharValue('\0'), 1, 37, "", "'\0'"),
            Token::new(
                TokenType::StringValue("Hello, World!".to_string()),
                1,
                42,
                "",
                "\"Hello, World!\"",
            ),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_numbers() {
        let input = "0 10 1234";
        let output = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", "0"),
            Token::new(TokenType::I32Value(10), 1, 3, "", "10"),
            Token::new(TokenType::I32Value(1234), 1, 6, "", "1234"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_escape_lines() {
        let input = r#" "\n \t \" " "#;
        let output = vec![Token::new(
            TokenType::StringValue("\n \t \" ".to_string()),
            1,
            2,
            "",
            "\"\\n \\t \\\" \"",
        )];
        test_input(input, &output);
    }

    #[test]
    fn lex_comments() {
        let input = "i32 // Hello World\n i32";
        let output = vec![
            Token::new(TokenType::I32, 1, 1, "", "i32"),
            Token::new(TokenType::I32, 2, 2, "", "i32"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_arithmetic() {
        let input = "\n1 2 +\n3 4 +\n*\nprint";
        let output = vec![
            Token::new(TokenType::I32Value(1), 2, 1, "", "1"),
            Token::new(TokenType::I32Value(2), 2, 3, "", "2"),
            Token::new(TokenType::Add, 2, 5, "", "+"),
            Token::new(TokenType::I32Value(3), 3, 1, "", "3"),
            Token::new(TokenType::I32Value(4), 3, 3, "", "4"),
            Token::new(TokenType::Add, 3, 5, "", "+"),
            Token::new(TokenType::Asterisk, 4, 1, "", "*"),
            Token::new(TokenType::Print, 5, 1, "", "print"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_boolean() {
        let input = r#"> < == != <= >= && || & |"#;
        let output = vec![
            Token::new(TokenType::Greater, 1, 1, "", ">"),
            Token::new(TokenType::Less, 1, 3, "", "<"),
            Token::new(TokenType::Equal, 1, 5, "", "=="),
            Token::new(TokenType::NotEqual, 1, 8, "", "!="),
            Token::new(TokenType::LessEqual, 1, 11, "", "<="),
            Token::new(TokenType::GreaterEqual, 1, 14, "", ">="),
            Token::new(TokenType::And, 1, 17, "", "&&"),
            Token::new(TokenType::Or, 1, 20, "", "||"),
            Token::new(TokenType::BitwiseAnd, 1, 23, "", "&"),
            Token::new(TokenType::BitwiseOr, 1, 25, "", "|"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_stack_operations() {
        let input = r#"rot3 dup drop over swap print"#;
        let output = vec![
            Token::new(TokenType::Rotate3, 1, 1, "", "rot3"),
            Token::new(TokenType::Duplicate, 1, 6, "", "dup"),
            Token::new(TokenType::Drop, 1, 10, "", "drop"),
            Token::new(TokenType::Over, 1, 15, "", "over"),
            Token::new(TokenType::Swap, 1, 20, "", "swap"),
            Token::new(TokenType::Print, 1, 25, "", "print"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_while_loop() {
        let input = r#"0 while dup 1 > {1 +}"#;
        let output = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", "0"),
            Token::new(TokenType::While, 1, 3, "", "while"),
            Token::new(TokenType::Duplicate, 1, 9, "", "dup"),
            Token::new(TokenType::I32Value(1), 1, 13, "", "1"),
            Token::new(TokenType::Greater, 1, 15, "", ">"),
            Token::new(TokenType::LeftBrace, 1, 17, "", "{"),
            Token::new(TokenType::I32Value(1), 1, 18, "", "1"),
            Token::new(TokenType::Add, 1, 20, "", "+"),
            Token::new(TokenType::RightBrace, 1, 21, "", "}"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_if() {
        let input = r#"0 if 1 > { "Less\n" print } else { "Greater\n" print }"#;
        let output = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", "0"),
            Token::new(TokenType::If, 1, 3, "", "if"),
            Token::new(TokenType::I32Value(1), 1, 6, "", "1"),
            Token::new(TokenType::Greater, 1, 8, "", ">"),
            Token::new(TokenType::LeftBrace, 1, 10, "", "{"),
            Token::new(
                TokenType::StringValue("Less\n".to_string()),
                1,
                12,
                "",
                "\"Less\\n\"",
            ),
            Token::new(TokenType::Print, 1, 21, "", "print"),
            Token::new(TokenType::RightBrace, 1, 27, "", "}"),
            Token::new(TokenType::Else, 1, 29, "", "else"),
            Token::new(TokenType::LeftBrace, 1, 34, "", "{"),
            Token::new(
                TokenType::StringValue("Greater\n".to_string()),
                1,
                36,
                "",
                "\"Greater\\n\"",
            ),
            Token::new(TokenType::Print, 1, 48, "", "print"),
            Token::new(TokenType::RightBrace, 1, 54, "", "}"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_func() {
        let input = r#"func test i32 i32 -> i32 { + }"#;
        let output = vec![
            Token::new(TokenType::Func, 1, 1, "", "func"),
            Token::new(TokenType::Identifier("test".to_string()), 1, 6, "", "test"),
            Token::new(TokenType::I32, 1, 11, "", "i32"),
            Token::new(TokenType::I32, 1, 15, "", "i32"),
            Token::new(TokenType::Arrow, 1, 19, "", "->"),
            Token::new(TokenType::I32, 1, 22, "", "i32"),
            Token::new(TokenType::LeftBrace, 1, 26, "", "{"),
            Token::new(TokenType::Add, 1, 28, "", "+"),
            Token::new(TokenType::RightBrace, 1, 30, "", "}"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_variables() {
        let input = r#"0 assign x { x @ print x 1 = }"#;
        let output = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", "0"),
            Token::new(TokenType::Assignment, 1, 3, "", "assign"),
            Token::new(TokenType::Identifier("x".to_string()), 1, 10, "", "x"),
            Token::new(TokenType::LeftBrace, 1, 12, "", "{"),
            Token::new(TokenType::Identifier("x".to_string()), 1, 14, "", "x"),
            Token::new(TokenType::Read, 1, 16, "", "@"),
            Token::new(TokenType::Print, 1, 18, "", "print"),
            Token::new(TokenType::Identifier("x".to_string()), 1, 24, "", "x"),
            Token::new(TokenType::I32Value(1), 1, 26, "", "1"),
            Token::new(TokenType::Assign, 1, 28, "", "="),
            Token::new(TokenType::RightBrace, 1, 30, "", "}"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_record() {
        let input = r#"record temp { i32 v1 i32 v2 } temp dup .v1 swap 1 .v2!"#;
        let output = vec![
            Token::new(TokenType::Record, 1, 1, "", "record"),
            Token::new(TokenType::Identifier("temp".to_string()), 1, 8, "", "temp"),
            Token::new(TokenType::LeftBrace, 1, 13, "", "{"),
            Token::new(TokenType::I32, 1, 15, "", "i32"),
            Token::new(TokenType::Identifier("v1".to_string()), 1, 19, "", "v1"),
            Token::new(TokenType::I32, 1, 22, "", "i32"),
            Token::new(TokenType::Identifier("v2".to_string()), 1, 26, "", "v2"),
            Token::new(TokenType::RightBrace, 1, 29, "", "}"),
            Token::new(TokenType::Identifier("temp".to_string()), 1, 31, "", "temp"),
            Token::new(TokenType::Duplicate, 1, 36, "", "dup"),
            Token::new(
                TokenType::RecordIdentifier("v1".to_string()),
                1,
                40,
                "",
                ".v1",
            ),
            Token::new(TokenType::Swap, 1, 44, "", "swap"),
            Token::new(TokenType::I32Value(1), 1, 49, "", "1"),
            Token::new(
                TokenType::RecordIdentifier("v2".to_string()),
                1,
                51,
                "",
                ".v2",
            ),
            Token::new(TokenType::Exclamation, 1, 54, "", "!"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn filename() {
        let lexer = Lexer::init(Config::blank(), PathBuf::from("test/test/test.concat"));

        assert_eq!(
            lexer.get_filename(&PathBuf::from("test/test/test.concat")),
            "test.concat"
        );

        assert_eq!(
            lexer.get_filename(&PathBuf::from("test/test.concat")),
            "../test.concat"
        );

        assert_eq!(
            lexer.get_filename(&PathBuf::from("test/test2/test.concat")),
            "../test2/test.concat"
        );

        assert_eq!(
            lexer.get_filename(&PathBuf::from("test/test/test3/test.concat")),
            "test3/test.concat"
        );

        assert_eq!(
            lexer.get_filename(&PathBuf::from("test/test/test2/test.concat")),
            "test2/test.concat"
        );
    }
}
