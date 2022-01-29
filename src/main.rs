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
        Subcommand::Init => {
            if let Err(_) = fs::create_dir("./plugin") {
                println!("Directory already exists!");
                exit(1)
            }
            if let Err(_) = fs::create_dir_all("./plugin/src/pmhaxe") {
                println!("Couldn't create src directory!");
                exit(1)
            }
            if let Err(_) = util::create_plugin_manifest() {
                println!("Couldn't create plugin manifest!");
                exit(1)
            }
            if let Err(_) = util::create_plugin_main() {
                println!("Couldn't create plugin main");
                exit(1)
            }
        }
    }
}
