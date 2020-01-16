#[test]
fn test_run() {
    let output = std::process::Command::new("target/debug/fpm")
                                        .output().unwrap();
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        panic!("Command failed.")
    }
}
