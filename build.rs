use std::process::Command;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let output = Command::new("sh").arg("build.sh").output().unwrap();
    let mut file = File::create("buildlog")?;
    file.write_all(String::from_utf8(output.stdout).unwrap().as_bytes()).unwrap();
    Ok(())
}