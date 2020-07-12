//! parsing
use palette::{Srgb, Srgba};

/// Parse a 3- or 6-digit hexadecimal color into Srgb
pub fn parse_hex_srgb(hex: &str) -> Option<Srgb> {
    // remove the preceding #
    let hex = hex.trim_start_matches("#");

    if hex.len() == 6 {
        let r = i32::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
        let g = i32::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
        let b = i32::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
        Some(Srgb::new(r, g, b))
    } else if hex.len() == 3 {
        let r = i32::from_str_radix(&hex[0..1], 16).ok()? as f32 / 15.0;
        let g = i32::from_str_radix(&hex[1..2], 16).ok()? as f32 / 15.0;
        let b = i32::from_str_radix(&hex[2..3], 16).ok()? as f32 / 15.0;
        Some(Srgb::new(r, g, b))
    } else {
        None
    }
}

pub fn parse_hex_srgba(hex: &str) -> Option<Srgba> {
    // remove the preceding #
    let hex = hex.trim_start_matches("#");

    if hex.len() == 8 {
        let r = i32::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
        let g = i32::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
        let b = i32::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
        let a = i32::from_str_radix(&hex[6..8], 16).ok()? as f32 / 255.0;
        Some(Srgba::new(r, g, b, a))
    } else if hex.len() == 4 {
        let r = i32::from_str_radix(&hex[0..1], 16).ok()? as f32 / 15.0;
        let g = i32::from_str_radix(&hex[1..2], 16).ok()? as f32 / 15.0;
        let b = i32::from_str_radix(&hex[2..3], 16).ok()? as f32 / 15.0;
        let a = i32::from_str_radix(&hex[3..4], 16).ok()? as f32 / 15.0;
        Some(Srgba::new(r, g, b, a))
    } else {
        None
    }
}
