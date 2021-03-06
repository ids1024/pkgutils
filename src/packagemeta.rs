use std::collections::BTreeMap;
use toml::{self, to_string, from_str};

#[derive(Serialize, Deserialize)]
pub struct PackageMeta {
    pub name: String,
    pub version: String,
    pub target: String,
}

impl PackageMeta {
    pub fn new(name: &str, version: &str, target: &str) -> Self {
        PackageMeta {
            name: name.to_string(),
            version: version.to_string(),
            target: target.to_string(),
        }
    }

    pub fn from_toml(text: &str) -> Result<Self, toml::de::Error> {
       from_str(text)
    }

    pub fn to_toml(&self) -> String {
        // to_string *should* be safe to unwrap for this struct
        to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct PackageMetaList {
    pub packages: BTreeMap<String, String>,
}

impl PackageMetaList {
    pub fn new() -> Self {
        PackageMetaList {
            packages: BTreeMap::new()
        }
    }

    pub fn from_toml(text: &str) -> Result<Self, toml::de::Error> {
       from_str(text)
    }

    pub fn to_toml(&self) -> String {
        // to_string *should* be safe to unwrap for this struct
        to_string(self).unwrap()
    }
}
