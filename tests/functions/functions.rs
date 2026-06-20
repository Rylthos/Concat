use assert_cmd::Command;

#[test]
fn functions_basic() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/functions/basic.concat")
        .assert()
        .success()
        .stdout("(4, 8)\n");
}

#[test]
fn functions_multiple_io() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/functions/multiple_io.concat")
        .assert()
        .success()
        .stdout("test0\n1");
}
