use assert_cmd::Command;

#[test]
fn problem_1() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/projectEuler/problem1.concat")
        .assert()
        .success()
        .stdout("233168\n");
}

#[test]
fn problem_2() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/projectEuler/problem2.concat")
        .assert()
        .success()
        .stdout("4613732\n");
}

#[test]
fn problem_4() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/projectEuler/problem4.concat")
        .assert()
        .success()
        .stdout("906609\n");
}
