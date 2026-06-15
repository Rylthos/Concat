use assert_cmd::Command;

#[test]
fn arithmetic() {
    let mut cmd = Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/arithmetic.concat")
        .assert()
        .success()
        .stdout("Value: 21\n");
}
