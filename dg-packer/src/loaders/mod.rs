use serde::Deserialize;

pub mod text_loader;
pub mod image_loader;
pub mod texture_loader;


#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub(crate) enum Filter {
    #[default]
    Linear,
    Nearest,
}

impl Into<image::imageops::FilterType> for Filter {
    fn into(self) -> image::imageops::FilterType {
        match self {
            Filter::Linear => image::imageops::FilterType::CatmullRom,
            Filter::Nearest => image::imageops::FilterType::Nearest,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub(crate) struct ScaledSize {
    scale: f32,
    #[serde(default)]
    filter: Filter,
}


#[derive(Deserialize, Debug, Clone, Copy)]
pub(crate) struct SpecifiedSize {
    width: u32,
    height: u32,
    #[serde(default)]
    filter: Filter,
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub(crate) enum Size {
    #[default]
    AsIs,
    Scaled(ScaledSize),
    Specified(SpecifiedSize),
}