use assert_cmd::Command;

use std::fs;

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

#[test]
fn syscall_file_read_std() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/syscall/good_file_read.concat")
        .assert()
        .success()
        .stdout("Hello, File!\n");
}

#[test]
fn syscall_file_write_std() {
    let test_path = |path, msg| {
        Command::cargo_bin("concat")
            .unwrap()
            .arg("tests/syscall/file_write.concat")
            .write_stdin(format!("{}\n{}\n", path, msg))
            .assert()
            .success()
            .stdout(format!("File path: String: Wrote: {}\n", msg));

        let contents = fs::read_to_string(path).unwrap();
        assert_eq!(contents, msg);
    };

    for i in 0..10 {
        test_path(
            format!("/tmp/file_write_{}.txt", i),
            format!("Hello File {}!", i),
        );
    }
}
