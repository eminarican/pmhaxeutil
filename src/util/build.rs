use super::{Result, ResultError};
use super::Ok;

pub fn start() -> ResultError {
    if let Err(message) = compile() {
        return Err(message)
    }

    if let Err(message) = copy_manifest() {
        return Err(message)
    }

    Ok()
}

pub fn clean() -> Result {
    println!("Cleaning build files!");
    super::file::delete_folder(String::from("/out"))
}

fn compile() -> ResultError {
    use std::process::Command;

    if super::file::get(String::from("./build.hxml")).is_none() {
        return Err(String::from("Build file doesn't exist!"))
    }

    if Command::new("haxe").arg("./build.hxml").status().is_err() {
        return Err(String::from("There's a problem with build file or project!"))
    }

    Ok()
}

fn copy_manifest() -> ResultError {
    return match super::file::read(String::from("plugin.json")) {
        None => Err(String::from("Plugin manifest doesn't exist")),
        Some(data) => super::to_result_err(super::file::write(
            String::from("out/plugin.yml"),
            super::manifest::CustomManifest::from_string(data).to_pocketmine().to_string()
        ), String::from("Couldn't copy manifest!"))
    }
}
