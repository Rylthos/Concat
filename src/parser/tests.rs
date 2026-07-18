#[cfg(test)]
mod tests {
    use crate::ast::node::*;
    use crate::builtins::basic_types::{BasicType, PtrType, UnionType};
    use crate::builtins::builtins::Builtin;
    use crate::config::config::Config;
    use crate::lexer::tokens::{PositionInfo, Token, TokenType};

    use crate::parser::parser::Parser;

    fn test(input: Vec<Token>, expected_tree: Region) {
        let mut parser = Parser::init(Config::blank(), input);
        match parser.parse() {
            Ok(r) => {
                assert_eq!(format!("{:?}", r), format!("{:?}", expected_tree));
            }
            Err(e) => {
                assert!(false, "{:?}", e);
            }
        }
    }

    fn create_position(line: usize, column: usize) -> PositionInfo {
        PositionInfo {
            line,
            column,
            file: "".to_string(),
        }
    }

    fn create_node(line: usize, column: usize, builtin: Builtin) -> AstNode {
        AstNode::Builtin(create_position(line, column), builtin)
    }

    fn create_literal(line: usize, column: usize, name: &str) -> Literal {
        Literal {
            position: create_position(line, column),
            literal: name.to_string(),
        }
    }

    #[test]
    fn parse_tree_normal() {
        let input = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", "0"),
            Token::new(TokenType::I32Value(1), 1, 3, "", "1"),
            Token::new(TokenType::Add, 1, 4, "", "+"),
            Token::new(TokenType::I32Value(2), 1, 5, "", "2"),
            Token::new(TokenType::I32Value(1), 1, 7, "", "1"),
            Token::new(TokenType::Subtract, 1, 8, "", "-"),
            Token::new(TokenType::Asterisk, 1, 9, "", "*"),
        ];
        let expected_tree = Region {
            region: vec![
                create_node(1, 1, Builtin::I32Value(0)),
                create_node(1, 3, Builtin::I32Value(1)),
                create_node(1, 4, Builtin::Add),
                create_node(1, 5, Builtin::I32Value(2)),
                create_node(1, 7, Builtin::I32Value(1)),
                create_node(1, 8, Builtin::Subtract),
                create_node(1, 9, Builtin::Multiply),
            ],
        };
        test(input, expected_tree);
    }

    #[test]
    fn parse_tree_if() {
        let input = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::If, 1, 1, "", ""),
            Token::new(TokenType::I32Value(10), 1, 1, "", ""),
            Token::new(TokenType::Greater, 1, 1, "", ""),
            Token::new(TokenType::LeftBrace, 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
        ];
        let expected_tree = Region::from_vec(vec![
            create_node(1, 1, Builtin::I32Value(0)),
            AstNode::If(IfNode {
                position: create_position(1, 1),
                if_region: vec![(
                    Region::from_vec(vec![
                        create_node(1, 1, Builtin::I32Value(10)),
                        create_node(1, 1, Builtin::Greater),
                    ]),
                    Region::new(),
                )],
                else_region: Region::new(),
            }),
        ]);
        test(input, expected_tree);
    }

    #[test]
    fn parse_tree_if_else() {
        let input = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::If, 1, 1, "", ""),
            Token::new(TokenType::I32Value(10), 1, 1, "", ""),
            Token::new(TokenType::Greater, 1, 1, "", ""),
            Token::new(TokenType::LeftBrace, 1, 1, "", ""),
            Token::new(TokenType::I32Value(2), 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
            Token::new(TokenType::Else, 1, 1, "", ""),
            Token::new(TokenType::LeftBrace, 1, 1, "", ""),
            Token::new(TokenType::I32Value(3), 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
        ];
        let expected_tree = Region::from_vec(vec![
            create_node(1, 1, Builtin::I32Value(0)),
            AstNode::If(IfNode {
                position: create_position(1, 1),
                if_region: vec![(
                    Region::from_vec(vec![
                        create_node(1, 1, Builtin::I32Value(10)),
                        create_node(1, 1, Builtin::Greater),
                    ]),
                    Region::from_vec(vec![create_node(1, 1, Builtin::I32Value(2))]),
                )],
                else_region: Region::from_vec(vec![create_node(1, 1, Builtin::I32Value(3))]),
            }),
        ]);
        test(input, expected_tree);
    }

    #[test]
    fn parse_tree_if_elseif_else() {
        let input = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::If, 1, 1, "", ""),
            Token::new(TokenType::Duplicate, 1, 1, "", ""),
            Token::new(TokenType::I32Value(10), 1, 1, "", ""),
            Token::new(TokenType::Greater, 1, 1, "", ""),
            Token::new(TokenType::LeftBrace, 1, 1, "", ""),
            Token::new(TokenType::I32Value(2), 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
            Token::new(TokenType::Else, 1, 1, "", ""),
            Token::new(TokenType::If, 1, 1, "", ""),
            Token::new(TokenType::Duplicate, 1, 1, "", ""),
            Token::new(TokenType::I32Value(20), 1, 1, "", ""),
            Token::new(TokenType::Greater, 1, 1, "", ""),
            Token::new(TokenType::LeftBrace, 1, 1, "", ""),
            Token::new(TokenType::I32Value(3), 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
            Token::new(TokenType::Else, 1, 1, "", ""),
            Token::new(TokenType::LeftBrace, 1, 1, "", ""),
            // Token::new(TokenType::Drop, 1, 1, "", ""),
            Token::new(TokenType::I32Value(4), 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
        ];
        let expected_tree = Region::from_vec(vec![
            create_node(1, 1, Builtin::I32Value(0)),
            AstNode::If(IfNode {
                position: create_position(1, 1),
                if_region: vec![
                    (
                        Region::from_vec(vec![
                            create_node(1, 1, Builtin::Duplicate),
                            create_node(1, 1, Builtin::I32Value(10)),
                            create_node(1, 1, Builtin::Greater),
                        ]),
                        Region::from_vec(vec![create_node(1, 1, Builtin::I32Value(2))]),
                    ),
                    (
                        Region::from_vec(vec![
                            create_node(1, 1, Builtin::Duplicate),
                            create_node(1, 1, Builtin::I32Value(20)),
                            create_node(1, 1, Builtin::Greater),
                        ]),
                        Region::from_vec(vec![create_node(1, 1, Builtin::I32Value(3))]),
                    ),
                ],
                else_region: Region::from_vec(vec![create_node(1, 1, Builtin::I32Value(4))]),
            }),
        ]);
        test(input, expected_tree);
    }

    #[test]
    fn parse_tree_while() {
        let input = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::While, 1, 1, "", ""),
            Token::new(TokenType::Duplicate, 1, 1, "", ""),
            Token::new(TokenType::I32Value(10), 1, 1, "", ""),
            Token::new(TokenType::Less, 1, 1, "", ""),
            Token::new(TokenType::LeftBrace, 1, 1, "", ""),
            Token::new(TokenType::I32Value(1), 1, 1, "", ""),
            Token::new(TokenType::Add, 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
        ];
        let expected_tree = Region::from_vec(vec![
            create_node(1, 1, Builtin::I32Value(0)),
            AstNode::While(WhileNode {
                position: create_position(1, 1),
                condition: Region::from_vec(vec![
                    create_node(1, 1, Builtin::Duplicate),
                    create_node(1, 1, Builtin::I32Value(10)),
                    create_node(1, 1, Builtin::Less),
                ]),
                region: Region::from_vec(vec![
                    create_node(1, 1, Builtin::I32Value(1)),
                    create_node(1, 1, Builtin::Add),
                ]),
            }),
        ]);
        test(input, expected_tree);
    }

    #[test]
    fn parse_ptr() {
        let input = vec![
            Token::new(TokenType::I32, 1, 1, "", "i32"),
            Token::new(TokenType::Asterisk, 1, 1, "", "*"),
        ];
        let expected_tree = Region::from_vec(vec![create_node(
            1,
            1,
            Builtin::BasicType(BasicType::Ptr(Box::new(PtrType {
                is_const: false,
                r#type: BasicType::I32,
            }))),
        )]);
        test(input, expected_tree);
    }

    #[test]
    fn parse_tree_function() {
        let input = vec![
            Token::new(TokenType::Func, 1, 1, "", ""),
            Token::new(TokenType::Identifier("test".to_string()), 1, 1, "", ""),
            Token::new(TokenType::I32, 1, 1, "", ""),
            Token::new(TokenType::I32, 1, 1, "", ""),
            Token::new(TokenType::Arrow, 1, 1, "", ""),
            Token::new(TokenType::I32, 1, 1, "", ""),
            Token::new(TokenType::LeftBrace, 1, 1, "", ""),
            Token::new(TokenType::Add, 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::I32Value(1), 1, 1, "", ""),
            Token::new(TokenType::Identifier("test".to_string()), 1, 1, "", ""),
            Token::new(TokenType::Print, 1, 1, "", ""),
        ];

        let expected_tree = Region::from_vec(vec![
            AstNode::FuncDecl(FuncDeclNode {
                position: create_position(1, 1),
                name: "test".to_string(),
                inputs: vec![BasicType::I32, BasicType::I32],
                outputs: vec![BasicType::I32],
                region: Region::from_vec(vec![create_node(1, 1, Builtin::Add)]),
            }),
            create_node(1, 1, Builtin::I32Value(0)),
            create_node(1, 1, Builtin::I32Value(1)),
            AstNode::Literal(create_literal(1, 1, "test")),
            create_node(1, 1, Builtin::Print),
        ]);

        test(input, expected_tree);
    }

    #[test]
    fn parse_tree_assign() {
        let input = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::Assignment, 1, 1, "", ""),
            Token::new(TokenType::Identifier("t1".to_string()), 1, 1, "", ""),
            Token::new(TokenType::LeftBrace, 1, 1, "", ""),
            Token::new(TokenType::Identifier("t1".to_string()), 1, 1, "", ""),
            Token::new(TokenType::Assign, 1, 1, "", ""),
            Token::new(TokenType::Identifier("t1".to_string()), 1, 1, "", ""),
            Token::new(TokenType::Read, 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
        ];

        let expected_tree = Region::from_vec(vec![
            create_node(1, 1, Builtin::I32Value(0)),
            AstNode::Assign(AssignNode {
                position: create_position(1, 1),
                labels: vec!["t1".to_string()],
                region: Region::from_vec(vec![
                    AstNode::Literal(create_literal(1, 1, "t1")),
                    create_node(1, 1, Builtin::Assign),
                    AstNode::Literal(create_literal(1, 1, "t1")),
                    create_node(1, 1, Builtin::Read),
                ]),
            }),
        ]);

        test(input, expected_tree);
    }

    #[test]
    fn parse_tree_record_create() {
        let input = vec![
            Token::new(TokenType::Record, 1, 1, "", ""),
            Token::new(TokenType::Identifier("test".to_string()), 1, 1, "", ""),
            Token::new(TokenType::LeftBrace, 1, 1, "", ""),
            Token::new(TokenType::I32, 1, 1, "", ""),
            Token::new(TokenType::Identifier("v1".to_string()), 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
        ];

        let expected_tree = Region::from_vec(vec![AstNode::RecordDecl(RecordDeclNode {
            position: create_position(1, 1),
            name: "test".to_string(),
            entries: vec![("v1".to_string(), BasicType::I32)],
        })]);

        test(input, expected_tree);
    }

    #[test]
    fn parse_tree_record_write_read() {
        let input = vec![
            Token::new(TokenType::Record, 1, 1, "", ""),
            Token::new(TokenType::Identifier("test".to_string()), 1, 1, "", ""),
            Token::new(TokenType::LeftBrace, 1, 1, "", ""),
            Token::new(TokenType::I32, 1, 1, "", ""),
            Token::new(TokenType::Identifier("v1".to_string()), 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
            Token::new(TokenType::Identifier("test".to_string()), 1, 1, "", ""),
            Token::new(TokenType::Exclamation, 1, 1, "", ""),
            Token::new(TokenType::I32Value(1), 1, 1, "", ""),
            Token::new(TokenType::RecordIdentifier("v1".to_string()), 1, 1, "", ""),
            Token::new(TokenType::Exclamation, 1, 1, "", ""),
            Token::new(TokenType::RecordIdentifier("v1".to_string()), 1, 1, "", ""),
        ];

        let expected_tree = Region::from_vec(vec![
            AstNode::RecordDecl(RecordDeclNode {
                position: create_position(1, 1),
                name: "test".to_string(),
                entries: vec![("v1".to_string(), BasicType::I32)],
            }),
            create_node(1, 1, Builtin::Record("test".to_string())),
            create_node(1, 1, Builtin::I32Value(1)),
            AstNode::WriteRecordElementIdentifier(create_literal(1, 1, "v1")),
            AstNode::RecordElementIdentifier(create_literal(1, 1, "v1")),
        ]);

        test(input, expected_tree);
    }

    #[test]
    fn parse_tree_union() {
        let input = vec![
            Token::new(TokenType::LeftSqBracket, 1, 1, "", ""),
            Token::new(TokenType::I32, 1, 1, "", ""),
            Token::new(TokenType::I32, 1, 1, "", ""),
            Token::new(TokenType::RightSqBracket, 1, 1, "", ""),
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::I32Value(1), 1, 1, "", ""),
            Token::new(TokenType::I32Value(2), 1, 1, "", ""),
            Token::new(TokenType::Union, 1, 1, "", ""),
        ];

        let expected_tree = Region::from_vec(vec![
            create_node(
                1,
                1,
                Builtin::BasicType(BasicType::Union(Box::new(UnionType {
                    types: vec![BasicType::I32, BasicType::I32],
                }))),
            ),
            create_node(1, 1, Builtin::I32Value(0)),
            create_node(1, 1, Builtin::I32Value(1)),
            create_node(1, 1, Builtin::I32Value(2)),
            create_node(1, 1, Builtin::Union),
        ]);
        test(input, expected_tree);
    }

    #[test]
    fn parse_define() {
        let input = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::Identifier("temp".to_string()), 1, 1, "", ""),
            Token::new(TokenType::Define, 1, 1, "", ""),
            Token::new(TokenType::Identifier("temp".to_string()), 1, 1, "", ""),
            Token::new(TokenType::Identifier("temp".to_string()), 1, 1, "", ""),
        ];

        let expected_tree = Region::from_vec(vec![
            create_node(1, 1, Builtin::I32Value(0)),
            AstNode::Literal(create_literal(1, 1, "temp")),
            create_node(1, 1, Builtin::Define),
            AstNode::Literal(create_literal(1, 1, "temp")),
            AstNode::Literal(create_literal(1, 1, "temp")),
        ]);

        test(input, expected_tree);
    }
}
