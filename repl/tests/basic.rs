#[macro_use]
extern crate pretty_assertions;

use std::process::{Command, Stdio};

#[test]
fn issue_365_run_io_from_command_line() {
    let gluon_path = std::env::var("CARGO_BIN_EXE_gluon").unwrap();
    let output = Command::new(&*gluon_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("GLUON_PATH", "..")
        .arg("tests/print.glu")
        .output()
        .unwrap_or_else(|err| panic!("{}\nWhen opening `{}`", err, gluon_path));

    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr != "" {
        panic!("{}", stderr);
    }
    assert_eq!(String::from_utf8_lossy(&output.stdout), "123\n");
}
