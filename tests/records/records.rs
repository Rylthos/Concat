use assert_cmd::Command;

#[test]
fn record_basic_typed() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/records/basic_typed.concat")
        .assert()
        .success()
        .stdout("10:20\n");
}

#[test]
fn record_basic() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/records/basic.concat")
        .assert()
        .success()
        .stdout("10:20\n");
}
