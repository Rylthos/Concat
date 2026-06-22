use assert_cmd::Command;

#[test]
fn array_basic() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/arrays/basic_array.concat")
        .assert()
        .success()
        .stdout(
            r#"9
8
7
6
5
4
3
2
1
0
"#,
        );
}
