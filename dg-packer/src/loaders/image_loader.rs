use std::path::Path;

use dg_resource::image::Image;
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
    // #[error("conversion error: `{0}`")]
    // Conversion(&'static str),
}

#[derive(Deserialize, Debug)]
pub struct ImageLoader {
    #[serde(default)]
    size: Size,
}

impl SourceLoader for ImageLoader {
    type Output = Image;
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
        let src_image = src_image.to_rgba8();

        let image = Image {
            rgba: src_image.to_vec(),
            width: src_image.width(),
            height: src_image.height(),
        };

        Ok(image)
    }
}