use super::{Ok, Result, ResultError};
use std::process::Command;

pub fn start() -> ResultError {
    if let Err(message) = create_folder() {
        return Err(message)
    }

    if let Err(message) = copy_utils() {
        return Err(message)
    }

    if let Err(message) = fix() {
        return Err(message)
    }

    if let Err(message) = pack() {
        return Err(message)
    }
    Ok()
}

pub fn create_folder() -> ResultError {
    if super::file::create_folder(String::from("build")).is_err() {
        return Err(String::from("Couldn't create build folder!"))
    }
    Ok()
}

pub fn copy_utils() -> ResultError {
    let packager = include_bytes!("../../res/build/package.php");
    let fixer = include_bytes!("../../res/build/fixer.phar");

    if super::file::write(String::from("build/package.php"), String::from(std::str::from_utf8(packager).unwrap())).is_err() {
        return Err(String::from("couldn't copy packaging util!"))
    }

    if super::file::write_bytes(String::from("build/fixer.phar"), fixer).is_err() {
        return Err(String::from("couldn't copy fixing util!"))
    }
    Ok()
}

pub fn fix() -> ResultError {
    let result = Command::new("php").args(vec!["./build/fixer.phar", "fix", "--allow-risky=yes", "--rules", "phpdoc_to_return_type,phpdoc_to_param_type",  "out"]).status();
    let _ = super::file::delete_file(String::from("package.php"));
    super::to_result_err(super::to_result(result), String::from("Couldn't fix plugin!"))
}

pub fn pack() -> ResultError {
    let result = Command::new("php").arg(String::from("./build/package.php")).status();
    let _ = super::file::delete_file(String::from("package.php"));
    super::to_result_err(super::to_result(result), String::from("Couldn't pack plugin!"))
}

pub fn clean() -> Result {
    super::file::delete_folder(String::from("build"))
}
