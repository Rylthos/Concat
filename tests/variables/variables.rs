use assert_cmd::Command;

#[test]
fn basic_variable() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/variables/basic_variable.concat")
        .assert()
        .success()
        .stdout("0 1\n");
}

#[test]
fn nested_variable() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/variables/nested_variable.concat")
        .assert()
        .success()
        .stdout("x: 0\ny: 1\ny after: 1\n");
}

#[test]
fn multi_variable() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/variables/multi_variable.concat")
        .assert()
        .success()
        .stdout("x: 0\ny: 1\nz: 2\ny: 100\ny': 3\ny: 100\n");
}

#[test]
fn variable_in_func() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/variables/variable_in_func.concat")
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

45
"#,
        );
}
