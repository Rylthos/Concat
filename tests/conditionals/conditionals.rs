use assert_cmd::Command;

#[test]
fn if_test() {
    let mut cmd = Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/conditionals/if.concat")
        .assert()
        .success()
        .stdout("Less 10: 0\n");
}

#[test]
fn if_else_test() {
    let mut cmd = Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/conditionals/if_else.concat")
        .assert()
        .success()
        .stdout("Greater 10: 10\n");
}

#[test]
fn if_elseif_elsetest() {
    let mut cmd = Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/conditionals/if_elseif_else.concat")
        .assert()
        .success()
        .stdout("Less 30: 20\nDone\n");
}
