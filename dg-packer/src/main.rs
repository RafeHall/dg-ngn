#![feature(path_file_prefix, random)]

mod loaders;
mod manifest;
mod packed_asset;
mod source_loader;

use std::path::PathBuf;

use loaders::{image_loader::ImageLoader, text_loader::TextLoader, texture_loader::TextureLoader};
use clap::{Parser, ValueEnum};
use manifest::Manifest;
use packed_asset::PackedAsset;
use rayon::prelude::*;
use source_loader::SourceLoaders;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("deserialization error: `{0}`")]
    De(#[from] toml::de::Error),
    #[error("serialization error: `{0}`")]
    TomlSer(#[from] toml::ser::Error),
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Dest {
    Directory,
    File,
}

impl std::fmt::Display for Dest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(Parser, Debug)]
struct Args {
    manifest: PathBuf,
    #[arg(long, alias = "dest", default_value_t = Dest::File)]
    destination: Dest,
    #[arg(short, long, default_value_t = false)]
    debug: bool,
    #[arg(short, long, default_value_t = false)]
    text_output: bool,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    
    let mut builder = env_logger::builder();
    if args.debug {
        builder.filter_level(log::LevelFilter::Debug);
    }
    builder.init();

    let source = std::fs::read_to_string(&args.manifest)?;
    let manifest: Manifest = toml::from_str(&source)?;

    let mut loaders = SourceLoaders::new();
    loaders.add::<TextLoader>("text");
    loaders.add::<ImageLoader>("image");
    loaders.add::<TextureLoader>("texture");
    
    let mut path = args.manifest.clone();
    path.pop();

    let assets: Vec<PackedAsset> = manifest.assets.into_par_iter().map(|asset| {
        let path = path.join(&asset.path);
        let string_path = asset.path.to_string_lossy();
        let name = asset.kind.clone() + ":" + &string_path;
        let (bytes, crc) = loaders.load(asset.kind, asset.options, asset.compress, &path);
        let packed_asset = PackedAsset::new(name, asset.compress, asset.tags, bytes, crc);
        packed_asset
    }).collect();

    match args.destination {
        Dest::Directory => for asset in assets {
            let prefix = args.manifest.file_prefix().expect("manifest has no prefix");
            let mut path = PathBuf::from(prefix);
            let mut asset_path = PathBuf::from(&asset.path);
            asset_path.pop();
            path.push(asset_path);
            std::fs::create_dir_all(&path)?;
            let mut packed_asset_path = PathBuf::from(&asset.path);
            if args.text_output {
                packed_asset_path.set_extension("toml");
            } else {
                packed_asset_path.set_extension("pck");
            }
            path.push(packed_asset_path);
            
            if args.text_output {
                let text = toml::to_string(&asset)?;
                std::fs::write(path, text)?;
            } else {
                let bytes = bitcode::encode(&asset);
                std::fs::write(path, bytes)?;
                
            }
        },
        Dest::File => {
            let mut path = PathBuf::from(args.manifest.file_name().expect("manifest has no filename"));
            
            if args.text_output {
                path.set_extension("toml");
                
                #[derive(serde::Serialize)]
                struct Assets {
                    #[serde(rename = "asset")]
                    assets: Vec<PackedAsset>,
                }

                let text = toml::to_string(&Assets { assets })?;
                std::fs::write(path, text)?;
            } else {
                path.set_extension("pck");

                let bytes = bitcode::encode(&assets);
                std::fs::write(path, bytes)?;
            }
        },
    }

    Ok(())
}

