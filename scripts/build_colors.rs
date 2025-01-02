//! Dependencies can be specified in the script file itself as follows:
//!
//! ```cargo
//! [dependencies]
//! convert_case = "0.6.0"
//! ```


use std::{fs::File, io::Write};

use convert_case::{Case, Casing};

const COLORS: &str = include_str!("colors.txt");

fn parse_hex(hex: &str) -> Result<u32, &'static str> {
    let hex = if hex.starts_with("#") { &hex[1..] } else { hex };
    
    if hex.len() != 6 && hex.len() != 8 {
        return Err("Invalid hex color: Length must be 6 characters (without alpha) or 8 characters (with alpha)");
    }

    let mut hex_int = u32::from_str_radix(hex, 16).map_err(|_| "Invalid hex color: Non-hexadecimal characters")?;
    if hex.len() == 6 {
        // If the hex string is 6 characters long, we add the alpha channel manually
        hex_int = (hex_int << 8) | 0xFF;
    }
    
    Ok(hex_int)
}

fn main() {
    let mut f = File::create("colors.rs").unwrap();

    let colors: Vec<(&str, u32)> = COLORS.lines()
        .filter(|l| !l.starts_with("#"))
        .map(|l| {
            let (name, hexcode) = l.split_once("\t").unwrap();

            let h: String = hexcode.split_whitespace().collect();

            (name, parse_hex(&h).unwrap())
        })
        .collect();

    f.write("mod colors {\n\tuse super::Color;\n\n".as_bytes()).unwrap();

    for (name, hex) in colors {
        let name = name.to_case(Case::ScreamingSnake).replace("/", "_SLASH_").replace("'", "");
        let l = format!("\tpub static {}: Color = Color::from_hex_int({:#X});\n", name, hex);

        f.write(l.as_bytes()).unwrap();
    }

    f.write("}".as_bytes()).unwrap();

    f.flush().unwrap();
}