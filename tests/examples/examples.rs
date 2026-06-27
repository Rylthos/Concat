use assert_cmd::Command;

#[test]
fn example_simplified_fibonacci() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/simple_fibonacci.concat")
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

#[test]
fn example_fizzbuzz() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/fizzbuzz.concat")
        .assert()
        .success()
        .stdout(
            r#"1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
16
17
Fizz
19
"#,
        );
}

#[test]
fn example_fizzbuzzfuzz() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/fizzbuzzfuzz.concat")
        .assert()
        .success()
        .stdout(
            r#"1
Fizz
Buzz
FizzFuzz
5
FizzBuzz
7
FizzFuzz
Buzz
Fizz
11
FizzBuzzFuzz
13
Fizz
Buzz
FizzFuzz
17
FizzBuzz
19
FizzFuzz
Buzz
Fizz
23
FizzBuzzFuzz
25
Fizz
Buzz
FizzFuzz
29
FizzBuzz
"#,
        );
}

#[test]
fn example_bubblesort() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/bubble_sort.concat")
        .assert()
        .success()
        .stdout(
            r#"Before: 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
After : 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
"#,
        );
}

#[test]
fn example_string() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/string_operations.concat")
        .assert()
        .success()
        .stdout(
            r#"1234: 4
"Hello," || " World!" = "Hello, World!": 13
"#,
        );
}

#[test]
fn example_rule110() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/rule110.concat")
        .assert()
        .success()
        .stdout(
            r#"                                                  ##
                                                ####
                                              ######
                                            ####  ##
                                          ##########
                                        ####      ##
                                      ######    ####
                                    ####  ##  ######
                                  ##############  ##
                                ####          ######
                              ######        ####  ##
                            ####  ##      ##########
                          ##########    ####      ##
                        ####      ##  ######    ####
                      ######    ########  ##  ######
                    ####  ##  ####    ##########  ##
                  ################  ####      ######
                ####            ########    ####  ##
              ######          ####    ##  ##########
            ####  ##        ######  ########      ##
          ##########      ####  ######    ##    ####
        ####      ##    ##########  ##  ####  ######
      ######    ####  ####      ################  ##
    ####  ##  ############    ####            ######
  ##############        ##  ######          ####  ##
"#,
        );
}

#[test]
fn example_calculator() {
    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/simple_calculator.concat")
        .write_stdin("123\n456\n+\n")
        .assert()
        .success()
        .stdout("Value 1:   Value 2:   Operation: Output:    579\n");

    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/simple_calculator.concat")
        .write_stdin("456\n123\n-\n")
        .assert()
        .success()
        .stdout("Value 1:   Value 2:   Operation: Output:    333\n");

    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/simple_calculator.concat")
        .write_stdin("456\n2\n*\n")
        .assert()
        .success()
        .stdout("Value 1:   Value 2:   Operation: Output:    912\n");

    Command::cargo_bin("concat")
        .unwrap()
        .arg("examples/simple_calculator.concat")
        .write_stdin("456\n2\n/\n")
        .assert()
        .success()
        .stdout("Value 1:   Value 2:   Operation: Output:    228\n");
}
