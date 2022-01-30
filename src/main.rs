mod app;
mod util;

use app::Subcommand;

use std::fs::File;
use std::process::exit;
use std::process::Command;

fn main() {
    let matches = app::new();
    match app::subcommand(&matches) {
        Subcommand::Build => {
            if File::open("./build.hxml").is_err() {
                println!("Build file doesn't exist!");
                exit(1)
            }
            if Command::new("haxe").arg("./build.hxml").status().is_err() {
                println!("There's a problem with build file or project!");
                exit(1)
            }
            if let Some(manifest) = util::get_custom_manifest() {
                let manifest = manifest.to_pocketmine();
                if util::create_plugin_manifest(manifest).is_err() {
                    println!("Couldn't create plugin manifest!");
                    exit(1)
                }
            } else {
                println!("Plugin manifest doesn't exist!");
                exit(1)
            }
            println!("Project builded!")
        }
        Subcommand::Init { name, path, version } => {
            if util::create_custom_manifest(name, path, version).is_err() {
                println!("Couldn't create plugin manifest!");
                exit(1)
            }
            if util::create_plugin_build_info(name, path).is_err() {
                println!("Couldn't create plugin build files");
                exit(1)
            }
            if util::create_plugin_main(name, path).is_err() {
                println!("Couldn't create plugin main");
                exit(1)
            }
            println!("Project {} is initialized!", name)
        }
    }
}
