use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PocketmineManifest {
    api: String,
    name: String,
    main: String,
    version: String
}

impl PocketmineManifest {
    pub fn to_string(mut self) -> String {
        serde_yaml::to_string(&mut self).unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomManifest {
    ver: String,
    name: String,
    main: String,
    virions: Vec<String>
}

impl CustomManifest {
    pub fn from_string(data: String) -> CustomManifest {
        serde_json::from_str(data.as_str()).unwrap()
    }

    pub fn to_string(mut self) -> String {
        serde_json::to_string(&mut self).unwrap()
    }

    pub fn to_pocketmine(self) -> PocketmineManifest {
        PocketmineManifest {
            api: String::from("4.0.0"),
            name: self.name,
            main: self.main.replace(".", "\\"),
            version: self.ver
        }
    }
}
