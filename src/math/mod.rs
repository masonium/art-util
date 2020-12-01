pub mod types;
pub mod line_intersect;
pub mod clipping;

pub use types::{Rect, Scalar, ToArray};
pub use line_intersect::{refract_dir, line_intersect_2d, implicit_ray_intersect_2d, orient_2d, PointTest};
pub use clipping::{ClipResult, clip_line};
