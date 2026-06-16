use assert_cmd::Command;

#[test]
fn functions_basic() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/functions/basic.concat")
        .assert()
        .success()
        .stdout("(3, 7)\n");
}
