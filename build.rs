use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("git")
        .stderr(Stdio::piped())
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", stdout);
}
