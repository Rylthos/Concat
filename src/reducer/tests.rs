#[cfg(test)]
mod tests {
    use crate::ast::raw_node::Region;
    use crate::ast::reduced_node::*;
    use crate::builtins::basic_types::{BasicType, BasicUnionType};
    use crate::builtins::reduced_builtins::ReducedBuiltin;
    use crate::config::config::Config;
    use crate::lexer::tokens::{PositionInfo, Token, TokenType};

    use crate::parser::parser::Parser;
    use crate::reducer::reducer::Reducer;

    fn test(input: Vec<Token>, expected_tree: ReducedRegion) {
        let mut parser = Parser::init(Config::blank(), input);
        let t = match parser.parse() {
            Ok(r) => r,
            Err(e) => {
                assert!(false, "{:?}", e);
                Region::new()
            }
        };
        let mut reducer = Reducer::init(Config::blank(), t);

        match reducer.reduce() {
            Ok(r) => {
                assert_eq!(format!("{:?}", r), format!("{:?}", expected_tree))
            }
            Err(e) => {
                assert!(false, "{:?}", e)
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

    fn create_node(line: usize, column: usize, builtin: ReducedBuiltin) -> ReducedAstNode {
        ReducedAstNode::Builtin(create_position(line, column), builtin)
    }

    #[test]
    fn reduce_tree_normal() {
        let input = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", "0"),
            Token::new(TokenType::I32Value(1), 1, 3, "", "1"),
            Token::new(TokenType::Add, 1, 4, "", "+"),
            Token::new(TokenType::I32Value(2), 1, 5, "", "2"),
            Token::new(TokenType::I32Value(1), 1, 7, "", "1"),
            Token::new(TokenType::Subtract, 1, 8, "", "-"),
            Token::new(TokenType::Asterisk, 1, 9, "", "*"),
        ];
        let expected_tree = ReducedRegion {
            region: vec![
                create_node(1, 1, ReducedBuiltin::I32Value(0)),
                create_node(1, 3, ReducedBuiltin::I32Value(1)),
                create_node(1, 4, ReducedBuiltin::Add),
                create_node(1, 5, ReducedBuiltin::I32Value(2)),
                create_node(1, 7, ReducedBuiltin::I32Value(1)),
                create_node(1, 8, ReducedBuiltin::Subtract),
                create_node(1, 9, ReducedBuiltin::Multiply),
            ],
        };
        test(input, expected_tree);
    }

    #[test]
    fn reduce_tree_if_elseif_else() {
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
            Token::new(TokenType::I32Value(4), 1, 1, "", ""),
            Token::new(TokenType::RightBrace, 1, 1, "", ""),
        ];
        let expected_tree = ReducedRegion::from_vec(vec![
            create_node(1, 1, ReducedBuiltin::I32Value(0)),
            ReducedAstNode::If(ReducedIfNode {
                position: create_position(1, 1),
                if_region: vec![
                    (
                        ReducedRegion::from_vec(vec![
                            create_node(1, 1, ReducedBuiltin::Duplicate),
                            create_node(1, 1, ReducedBuiltin::I32Value(10)),
                            create_node(1, 1, ReducedBuiltin::Greater),
                        ]),
                        ReducedRegion::from_vec(vec![create_node(
                            1,
                            1,
                            ReducedBuiltin::I32Value(2),
                        )]),
                    ),
                    (
                        ReducedRegion::from_vec(vec![
                            create_node(1, 1, ReducedBuiltin::Duplicate),
                            create_node(1, 1, ReducedBuiltin::I32Value(20)),
                            create_node(1, 1, ReducedBuiltin::Greater),
                        ]),
                        ReducedRegion::from_vec(vec![create_node(
                            1,
                            1,
                            ReducedBuiltin::I32Value(3),
                        )]),
                    ),
                ],
                else_region: ReducedRegion::from_vec(vec![create_node(
                    1,
                    1,
                    ReducedBuiltin::I32Value(4),
                )]),
            }),
        ]);
        test(input, expected_tree);
    }

    #[test]
    fn reduce_tree_union() {
        let input = vec![
            Token::new(TokenType::LeftSqBracket, 1, 1, "", ""),
            Token::new(TokenType::I32, 1, 1, "", ""),
            Token::new(TokenType::I32, 1, 1, "", ""),
            Token::new(TokenType::RightSqBracket, 1, 1, "", ""),
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::I32Value(1), 1, 1, "", ""),
            Token::new(TokenType::I32Value(2), 1, 1, "", ""),
            Token::new(TokenType::Union, 1, 1, "", ""),
            Token::new(TokenType::Duplicate, 1, 1, "", ""),
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::Nth, 1, 1, "", ""),
            Token::new(TokenType::Drop, 1, 1, "", ""),
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::I32Value(1), 1, 1, "", ""),
            Token::new(TokenType::Nth, 1, 1, "", ""),
            Token::new(TokenType::Exclamation, 1, 1, "", ""),
        ];

        let expected_tree = ReducedRegion::from_vec(vec![
            create_node(
                1,
                1,
                ReducedBuiltin::BasicType(BasicType::Union(Box::new(BasicUnionType {
                    types: vec![BasicType::I32, BasicType::I32],
                }))),
            ),
            create_node(1, 1, ReducedBuiltin::I32Value(0)),
            create_node(1, 1, ReducedBuiltin::I32Value(1)),
            create_node(1, 1, ReducedBuiltin::Union(2)),
            create_node(1, 1, ReducedBuiltin::Duplicate),
            create_node(1, 1, ReducedBuiltin::Nth(0)),
            create_node(1, 1, ReducedBuiltin::Drop),
            create_node(1, 1, ReducedBuiltin::I32Value(0)),
            create_node(1, 1, ReducedBuiltin::NthWrite(1)),
        ]);
        test(input, expected_tree);
    }

    #[test]
    fn reduce_define() {
        let input = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", ""),
            Token::new(TokenType::Identifier("temp".to_string()), 1, 1, "", ""),
            Token::new(TokenType::Define, 1, 1, "", ""),
            Token::new(TokenType::Identifier("temp".to_string()), 1, 1, "", ""),
            Token::new(TokenType::Identifier("temp".to_string()), 1, 1, "", ""),
        ];

        let expected_tree = ReducedRegion::from_vec(vec![
            create_node(1, 1, ReducedBuiltin::I32Value(0)),
            create_node(1, 1, ReducedBuiltin::I32Value(0)),
        ]);

        test(input, expected_tree);
    }
}
