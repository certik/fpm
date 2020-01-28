use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use assert_cmd::assert::Assert;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;

pub trait Success2 {
    // Our own function with better reporting of errors
    fn success2(self) -> Self;
}

#[cfg(unix)]
fn get_signal(status: ExitStatus) -> Option<i32> {
    status.signal()
}

#[cfg(not(unix))]
fn get_signal(_status: ExitStatus) -> Option<i32> {
    None
}

impl Success2 for Assert {
    fn success2(self) -> Self {
        if !self.get_output().status.success() {
            let output = self.get_output();
            let code = output.status.code();
            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            if cfg!(unix) {
                if code.is_none() {
                    let signal = get_signal(output.status).unwrap();
                    panic!("INTERRUPTED with signal: {}", signal);
                }
            }
            let actual_code = code.unwrap();
            println!("code: {}", actual_code);
            panic!("Non zero exit code");
        }
        self
    }
}

fn fpm_bin() -> Command {
    let mut fpm_bin_relative: std::path::PathBuf = ["target", "debug", "fpm"].iter().collect();
    fpm_bin_relative.set_extension(std::env::consts::EXE_EXTENSION);
    let fpm_bin_absolute = std::fs::canonicalize(fpm_bin_relative).unwrap();
    Command::new(fpm_bin_absolute.to_str().unwrap())
}

#[test]
fn test_help() {
    let mut cmd = fpm_bin();
    cmd.arg("--help");
    cmd.assert()
        .success2()
        .stdout(
            predicate::str::contains("--help       Prints help information"));
}


#[test]
fn test_1() {
    let mut build = fpm_bin();
    build.arg("build")
        .arg("--target-dir")
        .arg("build-test")
        .current_dir("tests/1");
    build.assert()
        .success2()
        .stdout(predicate::str::contains("Built target p1")
                .and(predicate::str::contains("TEST1 OK").not()));

    let mut run = fpm_bin();
    run.arg("run")
        .current_dir("tests/1");
    run.assert()
        .success2()
        .stdout(predicate::str::contains("TEST1 OK"));
}

#[test]
fn test_2() {
    let mut build = fpm_bin();
    build.arg("build")
        .arg("--target-dir")
        .arg("build-test")
        .current_dir("tests/2");
    build.assert()
        .success2()
        .stdout(predicate::str::contains("Built target p1")
                .and(predicate::str::contains("TEST2 OK").not()));

    let mut run = fpm_bin();
    run.arg("run")
        .current_dir("tests/2");
    run.assert()
        .success2()
        .stdout(predicate::str::contains("TEST2 OK"));
}
