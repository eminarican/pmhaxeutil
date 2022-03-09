use super::ResultError;
use super::Ok;

pub fn start(manifest: super::manifest::CustomManifest) -> ResultError {
    return if let Err(message) = create_build_info(&manifest) {
        Err(message)
    } else if let Err(message) = create_manifest(&manifest) {
        Err(message)
    } else if let Err(message) = create_main(&manifest) {
        Err(message)
    } else {
        Ok()
    }
}

fn create_build_info(manifest: &super::manifest::CustomManifest) -> ResultError {
    if super::file::create_folder(manifest.name.clone()).is_err() {
        return Err(String::from("Couldn't create folder!"))
    }

    let data = include_bytes!("../../res/build.hxml");

    let mut info = bytes_to_string(data);
    info = info.replace("pmhaxe.Main", manifest.main.as_str());

    super::to_result_err(
        super::file::write(format!("./{}/build.hxml", manifest.name), info),
        String::from("Couldn't create plugin build files!")
    )
}

fn create_manifest(manifest: &super::manifest::CustomManifest) -> ResultError {
    super::to_result_err(
        super::file::write(format!("./{}/plugin.json", manifest.name), manifest.to_string()),
        String::from("Couldn't create plugin manifest!")
    )
}

fn create_main(manifest: &super::manifest::CustomManifest) -> ResultError {
    let data = include_bytes!("../../res/src/pmhaxe/Main.hx");
    let path = manifest.name.clone() + "/src/";
    let mut info = bytes_to_string(data);

    if super::file::create_folder(path.clone() + manifest.get_namespace_fs().as_str()).is_err() {
        return Err(String::from("Couldn't create folder!"))
    }

    info = info.replace("pmhaxe", manifest.get_namespace().as_str());

    super::to_result_err(
        super::file::write(path + manifest.get_main_path().as_str(), info),
        String::from("Couldn't create plugin main")
    )
}

fn bytes_to_string(bytes: &[u8]) -> String {
    String::from(std::str::from_utf8(bytes).unwrap())
}
