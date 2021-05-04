mod color;
mod common;
pub mod easing;
mod fn_gen;
pub mod frustum;
mod image_pack;
mod image_util;
mod math;
pub mod models;
mod poisson;
mod random;
mod spatial_hash;
pub mod svg;
//mod dual_contour;

pub mod midi;

#[cfg(feature = "image_pack")]
pub use image_pack::{
    pack_as_vec, pack_into_vec, unpack_r_image_from_vec_1d, unpack_r_image_from_vec_2d,
    unpack_r_image_from_vec_3d, unpack_rg_image_from_vec_1d, unpack_rg_image_from_vec_2d,
    unpack_rg_image_from_vec_3d, unpack_rgb_image_from_vec_1d, unpack_rgb_image_from_vec_2d,
    unpack_rgb_image_from_vec_3d, unpack_rgba_image_from_vec_1d, unpack_rgba_image_from_vec_2d,
    unpack_rgba_image_from_vec_3d,
};

pub use crate::svg::{polygon_to_node, polyline_to_node};
pub use image_util::read_rgba_image_to_array;
pub use math::{clip_line, clip_polyline, ClipResult, Rect};
pub use math::{implicit_ray_intersect_2d, line_intersect_2d, orient_2d, PointTest};
pub use math::{refract_dir, ToArray};
pub use models::add_box;
pub use poisson::PoissonSampling;
pub use spatial_hash::SpatialHash2D;

pub use color::{parse_hex_srgb, parse_hex_srgba};
pub use fn_gen::gen_dated_filenames;
pub use frustum::Frustum;
pub use math::find_root;
pub use random::{random_quat, random_unit_vector};
//pub use dual_contour::{CellClass, QuadTree};
