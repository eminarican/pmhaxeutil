use clap::{App, ArgMatches};
use std::process::exit;

pub fn new() -> ArgMatches {
    App::new("Pmhaxe")
        .version("0.1.0")
        .about("Utils for pmhaxe")
        .subcommand(App::new("build")
            .about("Builds plugin")
        )
        .subcommand(App::new("init")
            .about("Creates project")
        )
        .get_matches()
}

pub enum Subcommand {
    Build,
    Init
}

pub fn subcommand(matches: &ArgMatches) -> Subcommand {
    return if let Some(_) = matches.subcommand_matches("build") {
        Subcommand::Build
    } else if let Some(_) = matches.subcommand_matches("init") {
        Subcommand::Init
    } else {
        println!("Please use --help flag for gathering more info");
        exit(0)
    }
}
