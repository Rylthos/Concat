use assert_cmd::Command;

#[test]
fn struct_basic() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/structs/basic.concat")
        .assert()
        .success()
        .stdout("10:20\n");
}
