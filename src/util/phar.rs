use super::{Result, Err, Ok};
use std::process::Command;

pub fn pack() -> Result {
    let data = include_bytes!("../../res/package.php");
    if super::file::write(String::from("package.php"), String::from(std::str::from_utf8(data).unwrap())).is_err() {
        return Err()
    }
    if Command::new("php").arg(super::file::path(String::from("package.php"))).status().is_err() {
        return Err()
    }
    Ok()
}
