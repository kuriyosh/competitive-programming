use cli_test_dir::*;

const BIN: &'static str = "./c";

fn test_runner(input: &str, expected: &str) {
    let test_dir = TestDir::new(BIN, "");
    let output = test_dir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), expected);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample1() {
    test_runner(
        r"7
    6 7 2 1 3 4 5",
        "4
7 5 3 2\n",
    )
}

#[test]
fn sample2() {
    test_runner(
        r"2
    2 1",
        "2
    1 2\n",
    )
}

#[test]
fn sample3() {
    test_runner(
        r"8
        3 7 4 7 3 3 8 2",
        "3
        2 7 8\n",
    )
}
