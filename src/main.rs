mod app;
mod util;

use app::Subcommand;

use std::process::exit;

fn main() {
    let matches = app::new();
    let subcommand = if let Some(subcommand) = app::subcommand(&matches) 
    {subcommand} else {
        exit_error(String::from("Please use --help flag for gathering more info"))
    };

    match subcommand {
        Subcommand::Build 
        => subcommand_build(),

        Subcommand::Init { name, path, version } 
        => subcommand_init(name, path, version)
    }
}

fn subcommand_build() {
    if let Err(message) = util::build::start() {
        if util::build::clean().is_err() {
            println!("Cleaning build files failed!")
        }
        exit_error(message)
    }

    if let Err(message) = util::phar::start() {
        if util::build::clean().is_err() || util::phar::clean().is_err() {
            println!("Cleaning build files failed!")
        }
        exit_error(message)
    }

    if util::build::clean().is_err() || util::phar::clean().is_err() {
        println!("Cleaning build files failed!")
    }
    
    exit_success(String::from("Project builded!"))
}

fn subcommand_init(name: String, path: Option<String>, version: Option<String>) {
    let folder = name.clone();
    let manifest = util::manifest::CustomManifest::new(name, path, version);

    if let Err(message) = util::init::start(manifest) {
        if util::file::delete_folder(folder).is_err() { 
            println!("Cleaning init files failed!")
        }
        exit_error(message)
    }
    
    exit_success(String::from("Project is initialized!"))
}

fn exit_success(message: String) {
    println!("{}", message);
    exit(0)
}

fn exit_error(message: String) -> ! {
    println!("{}", message);
    exit(1)
}
