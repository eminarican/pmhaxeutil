use std::fs;
use std::str;
use std::io::Write;

pub fn create_plugin_manifest(name: &str, path: Option<&str>, version: Option<&str>) -> Result<(), ()> {
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

pub fn create_plugin_build_info(name: &str, path: Option<&str>) -> Result<(), ()> {
    let data = include_bytes!("../res/build.hxml");
    
    let mut info = String::from(str::from_utf8(data).unwrap());
    if let Some(path) = path {
        info = info.replace("pmhaxe.", path);
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

    if let Err(_) = fs::create_dir_all(format!("./{}/src/{}", name, namespace)) {
        return Err(())
    }

    return create_file(format!("./{}/src/{}/Main.hx", name, namespace), main.as_bytes())
}

fn create_file(path: String, buff: &[u8]) -> Result<(), ()> {
    if let Ok(mut file) = fs::File::create(path) {
        if let Ok(_) = file.write_all(buff) {
            return Ok(())
        }
    }
    Err(())
}
