mod app;
mod util;

use app::Subcommand;

use std::process::exit;

fn main() {
    let matches = app::new();
    match app::subcommand(&matches) {
        Subcommand::Build => {
        }
        Subcommand::Init { name, path, version } => {
            if let Err(_) = util::create_plugin_manifest(name, path, version) {
                println!("Couldn't create plugin manifest!");
                exit(1)
            }
            if let Err(_) = util::create_plugin_build_info(name, path) {
                println!("Couldn't create plugin build files");
                exit(1)
            }
            if let Err(_) = util::create_plugin_main(name, path) {
                println!("Couldn't create plugin main");
                exit(1)
            }
        }
    }
}
