use assert_cmd::Command;

#[test]
fn define_basic() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/defines/basic_define.concat")
        .assert()
        .success()
        .stdout("0\n");
}
