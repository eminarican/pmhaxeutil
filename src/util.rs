use std::fs;
use std::str;
use std::io::{ Write, Read };

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PocketmineManifest {
    api: String,
    name: String,
    main: String,
    version: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomManifest {
    ver: String,
    name: String,
    main: String,
    virions: Vec<String>
}

impl CustomManifest {
    pub fn to_pocketmine(self) -> PocketmineManifest {
        PocketmineManifest {
            api: String::from("4.0.0"),
            name: self.name,
            main: self.main.replace(".", "\\"),
            version: self.ver
        }
    }
}

pub fn create_custom_manifest(name: &str, path: Option<&str>, version: Option<&str>) -> Result<(), ()> {
    let data = include_bytes!("../res/plugin.json");

    if let Err(_) = fs::create_dir(format!("./{}", name).as_str()) {
        println!("Directory already exists!");
        return Err(())
    }

    let mut manifest = String::from(str::from_utf8(data).unwrap());
    manifest = manifest.replace("HaxePlugin", name);

    if let Some(path) = path {
        manifest = manifest.replace("pmhaxe", path);
    }

    if let Some(version) = version {
        manifest = manifest.replace("0.0.1", version);
    }

    return create_file(format!("./{}/plugin.json", name), manifest.as_bytes())
}

pub fn create_plugin_manifest(manifest: PocketmineManifest) -> Result<(), ()> {
    let data = serde_yaml::to_string(&manifest).unwrap();
    return create_file(String::from("/out/plugin.yml"), data.as_bytes())
}

pub fn create_plugin_build_info(name: &str, path: Option<&str>) -> Result<(), ()> {
    let data = include_bytes!("../res/build.hxml");
    
    let mut info = String::from(str::from_utf8(data).unwrap());
    if let Some(path) = path {
        info = info.replace("pmhaxe.", format!("{}.", path).as_str());
    }

    return create_file(format!("./{}/build.hxml", name), info.as_bytes())
}

pub fn create_plugin_main(name: &str, path: Option<&str>) -> Result<(), ()> {
    let data = include_bytes!("../res/src/pmhaxe/Main.hx");
    
    let mut main = String::from(std::str::from_utf8(data).unwrap());

    let mut namespace = String::from("pmhaxe");
    if let Some(path) = path {
        namespace = path.replace(".", "/");
        main = main.replace("pmhaxe", namespace.as_str());
    }

    if fs::create_dir_all(format!("./{}/src/{}", name, namespace)).is_err() {
        return Err(())
    }

    return create_file(format!("./{}/src/{}/Main.hx", name, namespace), main.as_bytes())
}

pub fn get_custom_manifest() -> Option<CustomManifest> {
    if let Some(mut data) = get_file(String::from("plugin.json")) {
        if let Ok(manifest) = serde_json::from_str::<CustomManifest>(&mut data) {
            return Some(manifest)
        }
    }
    None
}

fn create_file(path: String, buff: &[u8]) -> Result<(), ()> {
    if let Ok(mut file) = fs::File::create(path) {
        if file.write_all(buff).is_ok() {
            return Ok(())
        }
    }
    Err(())
}

fn get_file(path: String) -> Option<String> {
    if let Ok(mut file) = fs::File::open(path) {
        let mut data = String::new();
        if file.read_to_string(&mut data).is_ok() {
            return Some(data)
        }
    }
    None
}
