use nalgebra::Vector3;
use num::{Float, Integer};
use std::convert::TryInto;

// extend an existing vertex and index list by a box
pub fn add_box<
    F: nalgebra::Scalar + nalgebra::RealField + num::Float,
    I: Integer + std::convert::TryFrom<usize>,
>(
    v: &mut Vec<Vector3<F>>,
    indices: &mut Vec<I>,
    center: Vector3<F>,
    half_widths: Vector3<F>,
) where
    <I as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
{
    let idx_offset: usize = v.len();
    const CUBE_INDEX_PATTERN: [usize; 36] = [
        0, 2, 1, 1, 2, 3, 1, 3, 5, 5, 3, 7, 4, 5, 6, 6, 5, 7, 0, 4, 2, 2, 4, 6, 2, 6, 3, 3, 6, 7,
        5, 4, 1, 1, 4, 0,
    ];
    for z in &[-1.0, 1.0] {
        for y in &[-1.0, 1.0] {
            for x in &[-1.0, 1.0] {
                let fx: F = center.x + F::from(*x).unwrap() * half_widths.x;
                let fy: F = center.y + F::from(*y).unwrap() * half_widths.y;
                let fz: F = center.z + F::from(*z).unwrap() * half_widths.z;

                v.push(Vector3::new(fx, fy, fz));
            }
        }
    }

    indices.extend(
        CUBE_INDEX_PATTERN
            .iter()
            .map(|i| (*i + idx_offset).try_into().unwrap()),
    );
}

/// Return `n` equally spaced values from start to end.
pub fn linspace<F: Float + 'static>(start: F, end: F, n: usize) -> impl Iterator<Item = F> {
    let df = (end - start) / F::from(n - 1).unwrap();
    (0..n).map(move |i| start + df.clone() * F::from(i).unwrap())
}

/// Append to the index list a set of indices for renderings lines,
/// starting at index `start` and rendering `n` points as consecutive lines.
/// (start, start+1, start+1, start+2, ..., start+n-2, start+n-1.
///
/// If `with_loop` is true, also append the indices from
pub fn add_linear_index(start: u32, n: u32, with_loop: bool) -> impl Iterator<Item = u32> {
    let last = 2 * if with_loop { n } else { n - 1 };
    (0..last).map(move |i| start + (i + 1) / 2)
}
