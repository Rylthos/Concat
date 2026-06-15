use assert_cmd::Command;

#[test]
fn basic_over() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/basics/over.concat")
        .assert()
        .success()
        .stdout("2 1 2 1 \n");
}

#[test]
fn basic_rotate() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/basics/rotate.concat")
        .assert()
        .success()
        .stdout(
            r#"Rotate 0: 3 2 1
Rotate 1: 2 1 3
Rotate 2: 1 3 2
Rotate 3: 3 2 1
"#,
        );
}
