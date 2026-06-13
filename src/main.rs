mod lexer;

fn main() {
    println!("Hello, world!");

    lexer::lexer::lexer(
        r#"
1 2 + // 3
3 4 + // 7
*     // 21

"Value: " print
string cast print
"\n" print

// "Value: 21\n"
        "#,
    );
}
