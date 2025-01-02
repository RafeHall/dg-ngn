use bitcode::{Decode, Encode};

use crate::Resource;


#[derive(Decode, Encode, Debug)]
pub struct Image {
    pub rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Resource for Image {}