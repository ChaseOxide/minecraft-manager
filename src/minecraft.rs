use std::{fmt::Display, process::Command};

pub fn minecraft_cmd<S: AsRef<str> + Display>(cmd: S) -> Result<std::process::Output, std::io::Error> {
    Command::new("screen")
        .arg("-p")
        .arg("0")
        .arg("-S")
        .arg("minecraft")
        .arg("-X")
        .arg("eval")
        .arg(format!("stuff \"{}\r\"", cmd))
        .output()
}
