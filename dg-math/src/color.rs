use std::ops::{Index, IndexMut};

use super::Scalar;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: Scalar,
    pub g: Scalar,
    pub b: Scalar,
    pub a: Scalar,
}

impl Color {
    pub const fn new(r: Scalar, g: Scalar, b: Scalar) -> Self {
        Self {
            r,
            g,
            b,
            a: 1.0,
        }
    }

    pub const fn new_alpha(r: Scalar, g: Scalar, b: Scalar, a: Scalar) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }
	
	pub const fn from_hex_int(hex: u32) -> Self {
		let r = ((hex >> 24) & 0xFF) as Scalar / 255.0;
		let g = ((hex >> 16) & 0xFF) as Scalar / 255.0;
		let b = ((hex >> 8) & 0xFF) as Scalar / 255.0;
		let a = (hex & 0xFF) as Scalar / 255.0;

		Self::new_alpha(r, g, b, a)
	}

    pub const fn from_rgba_bytes(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new_alpha(r as Scalar / 255.0, g as Scalar / 255.0, b as Scalar / 255.0, a as Scalar / 255.0)
    }

    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
		let hex = if hex.starts_with("#") { &hex[1..] } else { hex };

		if hex.len() != 6 && hex.len() != 8 {
			return Err("Invalid hex color: Length must be 6 characters (without alpha) or 8 characters (with alpha)");
		}

		let mut hex_int = u32::from_str_radix(hex, 16).map_err(|_| "Invalid hex color: Non-hexadecimal characters")?;
		if hex.len() == 6 {
			// If the hex string is 6 characters long, we add the alpha channel manually
			hex_int = (hex_int << 8) | 0xFF;
		}

		Ok(Self::from_hex_int(hex_int))
	}
}

include!("colors.rs");

impl Index<usize> for Color {
    type Output = Scalar;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!("Invalid index")
        }
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => panic!("Invalid index")
        }
    }
}