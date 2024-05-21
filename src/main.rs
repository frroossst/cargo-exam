use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .args(
            &[
            "test",
            "--color=always",
            "--", "--test-threads=1", 
            "--nocapture", "--color=always"
            ])
        .output()
        .expect("uh-oh! something went wrong :(");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
}

