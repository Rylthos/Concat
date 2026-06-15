use assert_cmd::Command;

#[test]
fn over() {
    let mut cmd = Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/basics/over.concat")
        .assert()
        .success()
        .stdout("2 1 2 1\n");
}
