use std::fs;
use std::str;
use std::io::Write;

pub fn create_plugin_manifest(name: &str, path: Option<&str>, version: Option<&str>) -> Result<(), ()> {
    let data = include_bytes!("../res/plugin.json");

    let mut manifest = String::from(std::str::from_utf8(data).unwrap());
    manifest = manifest.replace("HaxePlugin", name);

    if let Some(path) = path {
        manifest = manifest.replace("pmhaxe", path);
    }

    if let Some(version) = version {
        manifest = manifest.replace("0.0.1", version);
    }

    return create_file(format!("./{}/plugin.json", name).as_str(), manifest.as_bytes())
}

pub fn create_plugin_main(name: &str, path: Option<&str>) -> Result<(), ()> {
    let data = include_bytes!("../res/Main.hx");

    let mut namespace = String::from("pmhaxe");
    if let Some(path) = path {
        namespace = path.replace(".", "/")
    }

    if let Err(_) = fs::create_dir_all(format!("./{}/src/{}", name, namespace).as_str()) {
        return Err(())
    }

    return create_file(format!("./{}/src/{}/Main.hx", name, namespace).as_str(), data)
}

fn create_file(path: &str, buff: &[u8]) -> Result<(), ()> {
    if let Ok(mut file) = fs::File::create(path) {
        if let Ok(_) = file.write_all(buff) {
            return Ok(())
        }
    }
    Err(())
}
