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
