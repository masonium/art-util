pub mod clipping;
pub mod line_intersect;
pub mod root_finder;
pub mod types;

pub use clipping::{clip_line, clip_polyline, ClipResult};
pub use line_intersect::{
    implicit_ray_intersect_2d, line_intersect_2d, orient_2d, refract_dir, PointTest,
};
pub use root_finder::find_root;
pub use types::{Rect, Scalar, ToArray};
