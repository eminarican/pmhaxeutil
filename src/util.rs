use std::fs;
use std::io::Write;

pub fn create_plugin_manifest() -> Result<(), ()> {
    let data = include_bytes!("../res/plugin.json");
    return create_file("./plugin/plugin.json", data)
}

pub fn create_plugin_main() -> Result<(), ()> {
    let data = include_bytes!("../res/Main.hx");
    return create_file("./plugin/src/pmhaxe/Main.hx", data)
}

fn create_file(path: &str, buff: &[u8]) -> Result<(), ()> {
    if let Ok(mut file) = fs::File::create(path) {
        if let Ok(_) = file.write_all(buff) {
            return Ok(())
        }
    }
    Err(())
}
