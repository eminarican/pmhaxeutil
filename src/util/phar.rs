use super::{Ok, Result, ResultError};
use std::process::Command;

pub fn start() -> ResultError {
    return if let Err(message) = create_folder() {
        Err(message)
    } else if let Err(message) = copy_utils() {
        Err(message)
    } else if let Err(message) = fix() {
        Err(message)
    } else if let Err(message) = pack() {
        Err(message)
    } else {
        Ok()
    }
}

pub fn create_folder() -> ResultError {
    return if super::file::create_folder(String::from("build")).is_err() {
        Err(String::from("Couldn't create build folder!"))
    } else {
        Ok()
    }
}

pub fn copy_utils() -> ResultError {
    let packager = include_bytes!("../../res/build/package.php");
    let fixer = include_bytes!("../../res/build/fixer.phar");

    return if super::file::write(String::from("build/package.php"), String::from(std::str::from_utf8(packager).unwrap())).is_err() {
        Err(String::from("couldn't copy packaging util!"))
    } else if super::file::write_bytes(String::from("build/fixer.phar"), fixer).is_err() {
        Err(String::from("couldn't copy fixing util!"))
    } else {
        Ok()
    }
}

pub fn fix() -> ResultError {
    return match super::file::read(String::from("plugin.json")) {
        None => Err(String::from("Plugin manifest doesn't exist")),
        Some(data) => {
            let result_pm = Command::new("php").args(vec!["./build/fixer.phar", "fix", "--allow-risky=yes", "--rules", "phpdoc_to_return_type,phpdoc_to_param_type",  "out/src/lib/pocketmine"]).status();
            let result_pl = Command::new("php").args(vec!["./build/fixer.phar", "fix", "--allow-risky=yes", "--rules", "phpdoc_to_return_type,phpdoc_to_param_type",  format!("out/src/lib/{}", super::manifest::CustomManifest::from_string(data).get_namespace_fs()).as_str()]).status();
            if result_pm.is_ok() {
                let _ = super::file::delete_file(String::from("package.php"));
                super::to_result_err(super::to_result(result_pl), String::from("Couldn't fix plugin!"))
            } else {
                Err(String::from("Couldn't fix plugin!"))
            }
        }
    }
}

pub fn pack() -> ResultError {
    let result = Command::new("php").arg(String::from("./build/package.php")).status();
    let _ = super::file::delete_file(String::from("package.php"));
    super::to_result_err(super::to_result(result), String::from("Couldn't pack plugin!"))
}

pub fn clean() -> Result {
    let _ = super::file::delete_file(String::from(".php-cs-fixer.cache"));
    super::file::delete_folder(String::from("build"))
}
