use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
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

#[test]
fn test_help1() {
    for _i in 1..200 {
        let mut cmd = Command::new("target/debug/fpm");
        cmd.arg("--help");
        cmd.assert()
            .success2();
    }
}
