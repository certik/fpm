#[test]
fn test_run1() {
    let output = std::process::Command::new("target/debug/fpm")
                                        .output().unwrap();
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        panic!("Command failed.")
    }
}

#[test]
fn test_run2() {
    let output = std::process::Command::new("target/debug/fpm")
                                        .output().unwrap();
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        panic!("Command failed.")
    }
}

#[test]
fn test_run3() {
    let output = std::process::Command::new("target/debug/fpm")
                                        .output().unwrap();
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        panic!("Command failed.")
    }
}

#[test]
fn test_run4() {
    let output = std::process::Command::new("target/debug/fpm")
                                        .output().unwrap();
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        panic!("Command failed.")
    }
}

#[test]
fn test_run5() {
    let output = std::process::Command::new("target/debug/fpm")
                                        .output().unwrap();
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        panic!("Command failed.")
    }
}

#[test]
fn test_run6() {
    let output = std::process::Command::new("target/debug/fpm")
                                        .output().unwrap();
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        panic!("Command failed.")
    }
}
