use assert_cmd::Command;

#[test]
fn input_echo() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/input/echo.concat")
        .write_stdin("test\n")
        .assert()
        .success()
        .stdout("test\n ");
}
