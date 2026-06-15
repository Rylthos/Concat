use assert_cmd::Command;

#[test]
fn conditional_if() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/conditionals/if.concat")
        .assert()
        .success()
        .stdout("Less 10: 0\n");
}

#[test]
fn conditional_if_else() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/conditionals/if_else.concat")
        .assert()
        .success()
        .stdout("Greater 10: 10\n");
}

#[test]
fn conditional_if_elseif() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/conditionals/if_elseif_else.concat")
        .assert()
        .success()
        .stdout("Less 30: 20\nDone\n");
}

#[test]
fn conditional_nested_if() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/conditionals/nested_if.concat")
        .assert()
        .success()
        .stdout("Divisible by 4: 16\n");
}
