use assert_cmd::Command;

#[test]
fn basic_arithmetic() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/basics/arithmetic.concat")
        .assert()
        .success()
        .stdout("Value: 21\n");
}
