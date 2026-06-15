use assert_cmd::Command;
#[test]
fn simplified_fibonacci() {
    let mut cmd = Command::cargo_bin("concat")
        .unwrap()
        .arg("tests/fibonacci/simple_fibonacci.concat")
        .assert()
        .success()
        .stdout(
            r#"0: 0
1: 1
2: 1
3: 2
4: 3
5: 5
6: 8
7: 13
8: 21
9: 34
10: 55
11: 89
12: 144
13: 233
14: 377
15: 610
16: 987
17: 1597
18: 2584
19: 4181
20: 6765
"#,
        );
}
