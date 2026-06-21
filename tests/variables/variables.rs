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
