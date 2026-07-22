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

#[test]
fn syscall_file() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/syscall/openat.concat")
        .assert()
        .success()
        .stdout("Hello, File!\n");
}

#[test]
fn syscall_stat() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/syscall/stat.concat")
        .assert()
        .success()
        .stdout("13\n");
}
