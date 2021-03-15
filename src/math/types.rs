use nalgebra as na;
use na::{Point2, Vector2, center};
use ndarray_linalg::{Lapack, Scalar as NDScalar};
use std::fmt::Debug;

pub trait Scalar: na::RealField + Into<f64> + NDScalar + Lapack + Debug + Copy + 'static{}
impl<T> Scalar for T where T: na::RealField + Into<f64> + NDScalar + Lapack + Debug + Copy + 'static {}

/// Simple 2D rectangle, for common bounding-box style operations.
#[derive(Clone, Copy, Debug)]
pub struct Rect<F: Scalar> {
    pub p: [Point2<F>; 2]
}

impl<F: Scalar> Rect<F> {
    /// Create a rectangle from the extreme corner points.
    pub fn from_points(a: &Point2<F>, b: &Point2<F>) -> Rect<F> {
	Rect { p: [*a, *b] }
    }

    /// Create a rectangle from the minimum point and rectangle
    /// dimension.
    pub fn from_point_dim(a: &Point2<F>, d: &Vector2<F>) -> Rect<F> {
	Rect { p: [*a, *a + *d] }
    }

    /// Return the (width, height) of the rectangle as a 2d vector.
    pub fn dim(&self) -> Vector2<F> {
	self.p[1] - self.p[0]
    }

    /// Return the corners of the rect in (-x, -y), (x, -y) (-x, y),
    /// (x, y) order.
    pub fn corners(&self) -> [Point2<F>; 4] {
	[self.p[0], Point2::new(self.p[1].x, self.p[0].y),
	 self.p[1], Point2::new(self.p[0].x, self.p[1].y)]
    }

    pub fn midpoints(&self) -> [Point2<F>; 4] {
	let c = self.corners();
	[center(&c[0], &c[1]),
	 center(&c[1], &c[2]),
	 center(&c[2], &c[3]),
	 center(&c[3], &c[0])]
    }
    pub fn center(&self) -> Point2<F> {
	center(&self.p[0], &self.p[1])
    }

}

pub trait ToArray<T: Copy> {
    type Output;
    fn to_array(self) -> Self::Output;
}

impl<T: Copy> ToArray<T> for (T, T) {
    type Output = [T; 2];
    fn to_array(self) -> [T; 2] {
	[self.0, self.1]
    }
}

impl<T: Copy> ToArray<T> for (T, T, T) {
    type Output = [T; 3];
    fn to_array(self) -> [T; 3] {
	[self.0, self.1, self.2]
    }
}

impl<T: Copy> ToArray<T> for (T, T, T, T) {
    type Output = [T; 4];
    fn to_array(self) -> [T; 4] {
	[self.0, self.1, self.2, self.3]
    }
}
