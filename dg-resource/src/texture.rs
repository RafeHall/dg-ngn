use bitcode::{Decode, Encode};

use crate::Resource;


#[derive(Encode, Decode, Debug)]
pub struct Texture {

}

impl Resource for Texture {}