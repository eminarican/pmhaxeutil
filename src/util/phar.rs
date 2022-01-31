use super::{Result, Err};
use std::process::Command;

pub fn pack() -> Result {
    let data = include_bytes!("../../res/package.php");
    if super::file::write(String::from("package.php"), String::from(std::str::from_utf8(data).unwrap())).is_err() {
        return Err()
    }
    let result = Command::new("php").arg(String::from("package.php")).status();
    let _ = super::file::delete_file(String::from("package.php"));
    super::to_result(result)
}
