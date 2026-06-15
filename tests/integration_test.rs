use assert_cmd::Command;

#[test]
fn arithmetic() {
    let mut cmd = Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/arithmetic.concat")
        .assert()
        .success()
        .stdout("Value: 21\n");
}

// #[test]
// fn simplified_fibonacci() {
//     let mut cmd = Command::cargo_bin("concat")
//         .unwrap()
//         .arg("examples/simple_fibonacci.concat")
//         .assert()
//         .success()
//         .stdout("Value: 21\n");
// }

#[test]
fn if_test() {
    let mut cmd = Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/if.concat")
        .assert()
        .success()
        .stdout("Less 10: 0\n");
}
