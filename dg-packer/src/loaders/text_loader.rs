use std::path::Path;

use dg_resource::text::Text;
use serde::Deserialize;

use crate::source_loader::SourceLoader;

#[derive(Debug, Deserialize)]
pub struct TextLoader {}

impl SourceLoader for TextLoader {
    type Output = Text;
    type Error = std::io::Error;

    fn load(&self, path: &Path) -> Result<Self::Output, Self::Error> {
        Ok(Text {
            s: std::fs::read_to_string(path)?,
        })
    }
}
