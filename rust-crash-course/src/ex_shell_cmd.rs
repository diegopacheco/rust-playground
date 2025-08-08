use std::process::Command;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let out = Command::new("sh").arg("-c").arg("echo rust && uname -s").output()?;
    let s = String::from_utf8(out.stdout)?;
    let lines: Vec<&str> = s.lines().collect();
    println!("{}", lines.join(","));
    Ok(())
}
