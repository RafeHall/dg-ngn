use std::path::Path;

use rayon::prelude::*;
use dg_resource::texture::Texture;
use image::ColorType;
use log::{debug, warn};
use serde::Deserialize;
use thiserror::Error;

use crate::source_loader::SourceLoader;

use super::{ScaledSize, Size, SpecifiedSize};

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("image error: `{0}`")]
    Image(#[from] image::ImageError),
    #[error("unsupported color type: `{0:?}`")]
    UnsupportedColorType(ColorType),
}

fn yes() -> bool {
    true
}

#[derive(Deserialize, Debug)]
pub struct TextureLoader {
    #[serde(default = "yes")]
    mipmaps: bool,
    #[serde(default)]
    size: Size,
    #[serde(default = "yes")]
    compressed: bool,
}

impl SourceLoader for TextureLoader {
    type Output = Texture;
    type Error = Error;

    fn load(&self, path: &Path) -> Result<Self::Output, Self::Error> {
        let bytes = std::fs::read(path)?;
        let mut src_image = image::load_from_memory(&bytes)?;

        match self.size {
            Size::AsIs => {},
            Size::Scaled(ScaledSize { scale, filter }) => {
                let nwidth = (src_image.width() as f32 * scale) as u32;
                let nheight = (src_image.height() as f32 * scale) as u32;
                src_image = src_image.resize(nwidth, nheight, filter.into());
            },
            Size::Specified(SpecifiedSize { width: nwidth, height: nheight, filter }) => {
                src_image = src_image.resize(nwidth, nheight, filter.into());
            },
        }

        let width = src_image.width();
        let height = src_image.height();

        let mut generate_mipmaps = self.mipmaps;
        if self.mipmaps {
            if !width.is_power_of_two() {
                warn!("texture width is not power of two, skipping generating mipmaps: `{}`", width);
                generate_mipmaps = false;
            }
            if !height.is_power_of_two() {
                warn!("texture height is not power of two, skipping generating mipmaps: `{}`", height);
                generate_mipmaps = false;
            }
        }

        match src_image.color() {
            ColorType::L8 | ColorType::La8 | ColorType::Rgb8 | ColorType::Rgba8 => {}
            unsupported => return Err(Error::UnsupportedColorType(unsupported)),
        }

        let image = src_image.to_rgba8();

        let layers: Vec<Vec<u8>> = match generate_mipmaps {
            true => {
                let mipmap_layers = width.ilog2().min(height.ilog2());
                debug!("generating mipmap layers: `{}` - {}x{} to {}x{}", mipmap_layers, width, height, 1, 1);

                (0..=mipmap_layers).into_par_iter().map(|layer| {
                    // TODO: handle non-square sizes properly
                    let layer_width = 2_u32.pow(layer);
                    let layer_height = 2_u32.pow(layer);
                    let image = image::imageops::resize(&image, layer_width, layer_height, image::imageops::FilterType::CatmullRom);
                    image.to_vec()
                }).collect()
            },
            false => {
                vec![image.to_vec()]
            },
        };

        // for (i, layer) in layers.iter().enumerate() {
        //     println!("{i} {0}x{0}", (layer.len() as f64).sqrt() as usize / 2);
        // }

        // let compression_format = match src_image.color() {
        //     ColorType::L8 => todo!(),
        //     ColorType::La8 => todo!(),
        //     ColorType::Rgb8 => todo!(),
        //     ColorType::Rgba8 => todo!(),
        //     _ => unreachable!(),
        // };

        let texture = Texture {};

        Ok(texture)
    }
}
