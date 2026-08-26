use std::process::Command;

fn main() {
    let output = Command::new("rusqlite")
        .arg("sheffield1867.db")
        .arg("VACUUM")
        .output();

    match output {
        Ok(o) => {
            println!("Success: {}", String::from_utf8_lossy(&o.stdout));
            if !o.stderr.is_empty() {
                eprintln!("Errors: {}", String::from_utf8_lossy(&o.stderr));
            }
        }
        Err(e) => eprintln!("Failed to vacuum: {}", e),
    }
}
