use std::path::Path;

use nalgebra::{Vector4};
use num_traits::Zero;

#[derive(Debug)]
pub enum ReadImageError {
    ImageError(image::ImageError),
    FormatError
}

impl std::fmt::Display for ReadImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	match self {
	    Self::ImageError(e) => write!(f, "ImageError: {}", e),
	    Self::FormatError => write!(f, "image could not be read in desired format")
	}
    }
}

impl std::error::Error for ReadImageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
	match self {
	    Self::ImageError(e) => Some(e),
	    _ => None
	}
    }
}

impl From<image::ImageError> for ReadImageError {
    fn from(e: image::ImageError) -> ReadImageError {
	ReadImageError::ImageError(e)
    }
}

/// Return an RGBA image as an ndarray vector.
/// The y dimension is first.
pub fn read_rgba_image_to_array<P: AsRef<Path>>(
    path: P,
) -> Result<ndarray::Array2<Vector4<f32>>, ReadImageError> {
    let img = image::open(path)?;

    match img {
        image::DynamicImage::ImageRgb8(rgb_image) => {
            let width = rgb_image.width() as usize;
            let height = rgb_image.height() as usize;
            let mut arr = ndarray::Array2::from_elem((height, width), Vector4::zero());

            rgb_image.enumerate_pixels().for_each(|(x, y, p)| {
                arr[[y as usize, x as usize]] = Vector4::new(
                    p[0] as f32 / 255.0,
                    p[1] as f32 / 255.0,
                    p[2] as f32 / 255.0,
                    1.0,
                )
            });

            Ok(arr)
        }
        image::DynamicImage::ImageRgba8(rgba_image) => {
            let width = rgba_image.width() as usize;
            let height = rgba_image.height() as usize;
            let mut arr = ndarray::Array2::from_elem((height, width), Vector4::zero());

            rgba_image.enumerate_pixels().for_each(|(x, y, p)| {
                arr[[y as usize, x as usize]] = Vector4::new(
                    p[0] as f32 / 255.0,
                    p[1] as f32 / 255.0,
                    p[2] as f32 / 255.0,
                    p[3] as f32 / 255.0,
                )
            });

            Ok(arr)
        },
	_ => {
	    Err(ReadImageError::FormatError)
	}
    }
}
