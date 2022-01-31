use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PocketmineManifest {
    pub api: String,
    pub name: String,
    pub main: String,
    pub version: String
}

impl PocketmineManifest {
    pub fn to_string(mut self) -> String {
        serde_yaml::to_string(&mut self).unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomManifest {
    pub ver: String,
    pub name: String,
    pub main: String,
    pub virions: Vec<String>
}

impl CustomManifest {
    pub fn new(name: String, path: Option<String>, version: Option<String>) -> CustomManifest {
        let mut manifest = CustomManifest {
            ver: String::from("1.0.0"),
            name: name,
            main: String::from("pmhaxe.Main"),
            virions: vec![]
        };

        if let Some(data) = path {
            manifest.main = data + ".Main"
        }

        if let Some(data) = version {
            manifest.ver = data
        }

        manifest
    }

    pub fn from_string(data: String) -> CustomManifest {
        serde_json::from_str(data.as_str()).unwrap()
    }

    pub fn get_namespace(&self) -> String {
        self.main.replace(".Main", "")
    }

    pub fn get_namespace_fs(&self) -> String {
        self.get_namespace().replace(".", "/")
    }

    pub fn get_namespace_php(&self) -> String {
        self.get_namespace().replace(".", "\\")
    }

    pub fn get_main_path(&self) -> String {
        self.get_namespace_fs() + "/Main.hx"
    }

    pub fn get_main_php_namespace(&self) -> String {
        self.get_namespace_php() + "\\Main"
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn to_pocketmine(&self) -> PocketmineManifest {
        PocketmineManifest {
            api: String::from("4.0.0"),
            name: self.name.clone(),
            main: self.get_main_php_namespace(),
            version: self.ver.clone()
        }
    }
}
