use assert_cmd::Command;

#[test]
fn syscall_write() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/syscall/write.concat")
        .assert()
        .success()
        .stdout("Hello, World!\n");
}
