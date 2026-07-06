use std::process::Command;

#[test]
fn binary_prints_the_answer() {
    let out = Command::new(env!("CARGO_BIN_EXE_ci-sandbox-rust"))
        .output()
        .expect("binary should run");
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).expect("utf8 stdout");
    assert!(stdout.contains("the answer is 42"), "stdout was: {stdout}");
}
