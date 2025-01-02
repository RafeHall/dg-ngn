use std::collections::HashMap;

use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::manifest::Tag;

#[derive(Serialize, Deserialize, Encode, Decode, Debug)]
pub struct PackedAsset {
    pub path: String,
    pub compressed: bool,
    pub tags: HashMap<String, Tag>,
    pub bytes: Vec<u8>,
    pub crc: u32,
}

impl PackedAsset {
    pub fn new(path: String, compressed: bool, tags: HashMap<String, Tag>, bytes: Vec<u8>, crc: u32) -> Self {
        Self {
            path,
            compressed,
            tags,
            bytes,
            crc,
        }
    }
}