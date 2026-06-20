use assert_cmd::Command;

#[test]
fn basic_variables() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/variables/basic_variables.concat")
        .assert()
        .success()
        .stdout("0\n1\n");
}
