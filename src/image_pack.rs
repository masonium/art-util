//! Pack and unpack data between flat vectors of base components to
//! 'ndarray's of fixed-size vectors.
use nalgebra::{DimName, VectorN};
use ndarray as nd;
use nd::prelude::*;
use ndarray::{IntoDimension, Ix};
use std::fmt::Debug;
use std::iter::{FromIterator, IntoIterator};

pub enum PackImageError {
    Shape(nd::ShapeError),
    Contiguous,
}

impl From<nd::ShapeError> for PackImageError {
    fn from(e: nd::ShapeError) -> Self {
        PackImageError::Shape(e)
    }
}

pub trait DimExt: Dimension {
    fn reverse(self) -> Self;
    fn total(&self) -> usize;
}

impl DimExt for Dim<[Ix; 1]> {
    fn reverse(self) -> Self {
        self
    }
    fn total(&self) -> usize {
        self[0]
    }
}
impl DimExt for Dim<[Ix; 2]> {
    fn reverse(self) -> Self {
        Dim([self[1], self[0]])
    }
    fn total(&self) -> usize {
        self[0] * self[1]
    }
}

impl DimExt for Dim<[Ix; 3]> {
    fn reverse(self) -> Self {
        Dim([self[2], self[1], self[0]])
    }
    fn total(&self) -> usize {
        self[0] * self[1] * self[2]
    }
}

trait ImageDim: IntoDimension {
    const DIM: usize;
}

impl ImageDim for usize {
    const DIM: usize = 1;
}
impl ImageDim for (usize, usize) {
    const DIM: usize = 2;
}
impl ImageDim for (usize, usize, usize) {
    const DIM: usize = 3;
}

pub trait PixelDim: Sized {
    const DIM: usize;
    fn instance() -> Self;
}

impl PixelDim for nalgebra::U1 {
    const DIM: usize = 1;
    fn instance() -> Self {
        nalgebra::U1 {}
    }
}
impl PixelDim for nalgebra::U2 {
    const DIM: usize = 2;
    fn instance() -> Self {
        nalgebra::U2 {}
    }
}
impl PixelDim for nalgebra::U3 {
    const DIM: usize = 3;
    fn instance() -> Self {
        nalgebra::U3 {}
    }
}
impl PixelDim for nalgebra::U4 {
    const DIM: usize = 4;
    fn instance() -> Self {
        nalgebra::U4 {}
    }
}

/// reshape and pack an image from a flat vector to an nd with vector elements.
macro_rules! define_unpack_func {
    ($func_name:ident, $image_dim:ty,
     $pdim_type:ty) => {
        pub fn $func_name<T: Debug + Copy + PartialEq + 'static>(
            data: Vec<T>,
            dim: $image_dim,
        ) -> Result<
            nd::Array<VectorN<T, $pdim_type>, Dim<[Ix; <$image_dim>::DIM]>>,
            PackImageError,
        >
        where
            nalgebra::DefaultAllocator: nalgebra::base::allocator::Allocator<T, $pdim_type>,
        {
            let into_dim = dim.into_dimension();
            // let num_array_elem = into_dim.total();

            Array::from_iter(
                data.into_iter()
                    .as_slice()
                    .chunks(<$pdim_type>::DIM)
                    .map(|a| {
                        VectorN::from_row_slice_generic(
                            <$pdim_type>::instance(),
                            nalgebra::U1 {},
                            a,
                        )
                    }),
            )
            .into_shape(into_dim.reverse())
            .map_err(|x| x.into())
        }
    };
}

/// Return a flat vector from an array representing the image,
/// consuming the array. If the array is in standard layout, no
/// copying will occur.
///
/// The array is assumed to be in standard descending-coordinates
/// order (e.g. [z, y, x] for 3D).
pub fn pack_into_vec<
    T: PartialEq + Clone + Copy + std::fmt::Debug + 'static,
    AD: Dimension,
    PD: PixelDim + DimName,
>(
    arr: nd::Array<VectorN<T, PD>, AD>,
) -> Vec<T>
where
    nalgebra::DefaultAllocator: nalgebra::base::allocator::Allocator<T, PD>,
{
    if arr.is_standard_layout() {
        let x = arr.into_raw_vec();
        unsafe {
            let mut me = std::mem::ManuallyDrop::new(x);
            // Vector<T, PD> with the base allocator is just a [T; _]
            // underneath, and is thus guaranteed to have the same layout as T.
            Vec::from_raw_parts(
                me.as_mut_ptr() as *mut T,
                me.len() * PD::DIM,
                me.capacity() * PD::DIM,
            )
        }
    } else {
        pack_as_vec(&arr.view())
    }
}

/// Return a flat vector from an array representing the image.
///
/// The array is assumed to be in standard descending-coordinates
/// order (e.g. [z, y, x] for 3D).
pub fn pack_as_vec<
    T: PartialEq + Clone + Copy + std::fmt::Debug + 'static,
    AD: Dimension,
    PD: PixelDim + DimName,
>(
    arr: &nd::ArrayView<VectorN<T, PD>, AD>,
) -> Vec<T>
where
    nalgebra::DefaultAllocator: nalgebra::base::allocator::Allocator<T, PD>,
{
    let std_layout = arr.as_standard_layout();
    let x = std_layout.to_owned().into_raw_vec();
    Vec::from_iter(x.iter().flatten().cloned())
}

define_unpack_func!(unpack_r_image_from_vec_1d, usize, nalgebra::U1);

define_unpack_func!(unpack_rg_image_from_vec_1d, usize, nalgebra::U2);

define_unpack_func!(unpack_rgb_image_from_vec_1d, usize, nalgebra::U3);

define_unpack_func!(unpack_rgba_image_from_vec_1d, usize, nalgebra::U4);

define_unpack_func!(unpack_r_image_from_vec_2d, (usize, usize), nalgebra::U1);

define_unpack_func!(unpack_rg_image_from_vec_2d, (usize, usize), nalgebra::U2);

define_unpack_func!(unpack_rgb_image_from_vec_2d, (usize, usize), nalgebra::U3);

define_unpack_func!(unpack_rgba_image_from_vec_2d, (usize, usize), nalgebra::U4);

define_unpack_func!(
    unpack_r_image_from_vec_3d,
    (usize, usize, usize),
    nalgebra::U1
);

define_unpack_func!(
    unpack_rg_image_from_vec_3d,
    (usize, usize, usize),
    nalgebra::U2
);

define_unpack_func!(
    unpack_rgb_image_from_vec_3d,
    (usize, usize, usize),
    nalgebra::U3
);

define_unpack_func!(
    unpack_rgba_image_from_vec_3d,
    (usize, usize, usize),
    nalgebra::U4
);

#[cfg(test)]
mod test {
    use super::*;
    use na::Vector2;
    use na::Vector3;
    use nalgebra as na;

    #[test]
    fn pack_test_v2_f32() {
        let arr = nd::arr2(&[
            [Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0)],
            [Vector2::new(5.0, 6.0), Vector2::new(7.0, 8.0)],
            [Vector2::new(9.0, 10.0), Vector2::new(11.0, 12.0)],
        ]);
        let v = pack_into_vec(arr.clone());

        for i in 0..3 {
            for j in 0..2 {
                for c in 0..2 {
                    let idx = i * 4 + j * 2 + c;
                    assert_eq!(v[idx], arr[[i, j]][c]);
                }
            }
        }
    }

    #[test]
    fn pack_test_v3_f32() {
        let arr = nd::arr2(&[
            [Vector3::new(1.0, 2.0, 3.0), Vector3::new(3.0, 4.0, 5.0)],
            [Vector3::new(5.0, 6.0, 7.0), Vector3::new(7.0, 8.0, 9.0)],
        ]);
        let v = pack_into_vec(arr.clone());

        for i in 0..2 {
            for j in 0..2 {
                for c in 0..3 {
                    let idx = i * 6 + j * 3 + c;
                    assert_eq!(v[idx], arr[[i, j]][c]);
                }
            }
        }
    }
}
