use bitcode::{Decode, Encode};

use crate::Resource;


#[derive(Encode, Decode, Debug)]
pub struct Text {
    pub s: String,
}

impl Resource for Text {}