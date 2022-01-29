use clap::{App, Arg, ArgMatches};
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
            .arg(Arg::new("name")
                .value_name("name")
                .required(true)
                .short('n')
            )
            .arg(Arg::new("path")
                .value_name("path")
                .required(false)
                .short('p')
            )
            .arg(Arg::new("version")
                .value_name("version")
                .required(false)
                .short('v')
            )
        )
        .get_matches()
}

pub enum Subcommand<'a> {
    Build,
    Init {
        name: &'a str,
        path: Option<&'a str>,
        version: Option<&'a str>
    }
}

pub fn subcommand(matches: &ArgMatches) -> Subcommand {
    return if let Some(_) = matches.subcommand_matches("build") {
        Subcommand::Build
    } else if let Some(matches) = matches.subcommand_matches("init") {
        Subcommand::Init {
            name: matches.value_of("name").unwrap(),
            path: if let Some(path) = matches.value_of("path") {
                Some(path)
            } else {
                None
            },
            version: if let Some(version) = matches.value_of("version") {
                Some(version)
            } else {
                None
            }
        }
    } else {
        println!("Please use --help flag for gathering more info");
        exit(0)
    }
}
