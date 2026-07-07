use assert_cmd::Command;

#[test]
fn include_single() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/includes/single_include.concat")
        .assert()
        .success()
        .stdout("Hello, World!\n");
}

#[test]
fn include_multiple() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/includes/multiple_include.concat")
        .assert()
        .success()
        .stdout("Hello, World!\nHello, World, Again!\n");
}

#[test]
fn include_dir() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/includes/dir_include.concat")
        .assert()
        .success()
        .stdout("Hello, World!\nHello, World, Again!\n!dlroW ,olleH\n3\n");
}
