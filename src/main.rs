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

    if util::phar::pack().is_err() {
        if util::build::clean().is_err() {
            println!("Cleaning build files failed!")
        }
        exit_error(String::from("Couldn't pack plugin!"))
    }
    
    exit_success(String::from("Project builded!"))
}

fn subcommand_init(name: String, path: Option<String>, version: Option<String>) {
    /*if util::create_custom_manifest(name, path, version).is_err() {
        exit_error("Couldn't create plugin manifest!")
    }

    if util::create_plugin_build_info(name, path).is_err() {
        exit_error("Couldn't create plugin build files")
    }

    if util::create_plugin_main(name, path).is_err() {
        exit_error("Couldn't create plugin main")
    }*/
    
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
