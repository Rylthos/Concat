use assert_cmd::Command;

#[test]
fn loop_while() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/loops/while.concat")
        .assert()
        .success()
        .stdout(
            r#"0
1
2
3
4
5
6
7
8
9
"#,
        );
}
