#[test]
fn runs_without_arguments() {
    let output = std::process::Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
}

#[test]
fn runs_with_arguments() {
    let output = std::process::Command::new("cargo")
        .args(&["run", "a", "b", "c"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
}
