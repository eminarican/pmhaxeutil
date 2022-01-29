mod app;
mod util;

use std::process::exit;
use std::fs;

use app::Subcommand;

fn main() {
    let matches = app::new();
    match app::subcommand(&matches) {
        Subcommand::Build => {
        }
        Subcommand::Init { name, path, version } => {
            if let Err(_) = fs::create_dir(format!("./{}", name).as_str()) {
                println!("Directory already exists!");
                exit(1)
            }
            if let Err(_) = util::create_plugin_manifest(name, path, version) {
                println!("Couldn't create plugin manifest!");
                exit(1)
            }
            if let Err(_) = util::create_plugin_main(name, path) {
                println!("Couldn't create plugin main");
                exit(1)
            }
        }
    }
}
