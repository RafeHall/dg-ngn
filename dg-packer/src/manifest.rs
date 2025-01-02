use std::{collections::HashMap, path::PathBuf};

use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
#[serde(untagged)]
pub enum Tag {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

fn yes() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestAsset {
    pub kind: String,
    pub path: PathBuf,
    #[serde(default = "yes")]
    pub compress: bool,
    #[serde(flatten)]
    pub options: Table,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub tags: HashMap<String, Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub name: String,
    #[serde(rename = "asset")]
    pub assets: Vec<ManifestAsset>,
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        path::{Path, PathBuf},
    };

    use toml::Table;

    use crate::{loaders::text_loader::TextLoader, manifest::ManifestAsset, source_loader::SourceLoaders};

    use super::Manifest;

    #[test]
    fn serialize() {
        let manifest = Manifest {
            name: "test_manifest".into(),
            assets: vec![
                ManifestAsset {
                    kind: "Text".into(),
                    path: PathBuf::new(),
                    compress: false,
                    options: Table::default(),
                    tags: HashMap::default(),
                },
                ManifestAsset {
                    kind: "Image".into(),
                    path: PathBuf::new(),
                    compress: false,
                    options: Table::default(),
                    tags: HashMap::default(),
                },
                ManifestAsset {
                    kind: "Text".into(),
                    path: PathBuf::new(),
                    compress: false,
                    options: Table::default(),
                    tags: HashMap::default(),
                },
            ],
        };

        let text = toml::to_string_pretty(&manifest).unwrap();
        println!("{}", text);
    }

    #[test]
    fn deserialize() {
        let source = include_str!("test/test.toml");
        let manifest: Manifest = toml::from_str(source).unwrap();

        let mut loaders = SourceLoaders::new();
        loaders.add::<TextLoader>("Text");
        // loaders.add::<TextLoader>("Text");

        println!("{:#?}", manifest);

        let path = Path::new("src/packer/test");
        for asset in manifest.assets {
            let value = loaders.load(
                asset.kind.clone(),
                asset.options,
                asset.compress,
                &path.join(asset.path),
            );
            println!("{:#?}", value);
        }
    }
}
