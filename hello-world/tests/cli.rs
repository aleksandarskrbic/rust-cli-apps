use assert_cmd::Command;

#[test]
fn first_test() {
    assert!(true);
}

#[test]
fn ls_command() {
    let res = Command::new("ls").output();
    assert!(res.is_ok());
}

#[test]
fn hello() {
    let mut cmd = Command::cargo_bin("hello-world").unwrap();
    cmd.assert().success();
}

#[test]
fn hello_runs() {
    let mut cmd = Command::cargo_bin("hello-world").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

