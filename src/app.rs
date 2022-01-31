use clap::{App, Arg, ArgMatches};

pub fn new() -> ArgMatches {
    App::new("Pmhaxe")
        .version("0.1.0")
        .about("Utils for pmhaxe")
        .subcommand(App::new("build")
            .about("Builds plugin")
        )
        .subcommand(App::new("init")
            .about("Creates project")
            .arg(Arg::new("n")
                .value_name("name")
                .required(true)
                .short('n')
            )
            .arg(Arg::new("p")
                .value_name("path")
                .required(false)
                .short('p')
            )
            .arg(Arg::new("v")
                .value_name("version")
                .required(false)
                .short('v')
            )
        )
        .get_matches()
}

pub enum Subcommand {
    Build,
    Init {
        name: String,
        path: Option<String>,
        version: Option<String>
    }
}

pub fn subcommand(matches: &ArgMatches) -> Option<Subcommand> {
    if matches.subcommand_matches("build").is_some() {
        return Some(Subcommand::Build)
    }

    if let Some(matches) = matches.subcommand_matches("init") {
        return Some(Subcommand::Init {
            name: get_argument(matches, "n"),
            path: get_optional_argument(matches, "p"),
            version: get_optional_argument(matches, "v")
        })
    }

    None
}

fn get_argument(matches: &ArgMatches, value: &str) -> String {
    matches.value_of(value).unwrap().to_string()
}

fn get_optional_argument(matches: &ArgMatches, value: &str) -> Option<String> {
    return match matches.value_of(value) {
        Some(arg) => {
            Some(String::from(arg))
        },
        None => {
            None
        }
    }
}
