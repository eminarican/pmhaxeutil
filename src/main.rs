mod app;
mod util;

use app::Subcommand;

fn main() {
    let matches = app::new();
    match app::subcommand(&matches) {
        Subcommand::Build => {
        }
        Subcommand::Init => {
        }
    }
}
