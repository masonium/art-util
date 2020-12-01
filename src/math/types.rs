use nalgebra as na;
use na::{Point2, Vector2};
use std::fmt::Debug;
use std::convert::TryFrom;

pub trait Scalar: na::RealField + Into<f64> + TryFrom<f64> + Debug + Copy + 'static{}
impl<T> Scalar for T where T: na::RealField + Into<f64> + TryFrom<f64> + Debug + Copy + 'static {}

pub struct Rect<F: na::RealField> {
    pub p: [Point2<F>; 2]
}

impl<F: na::RealField> Rect<F> {
    pub fn from_points(a: &Point2<F>, b: &Point2<F>) -> Rect<F> {
	Rect { p: [*a, *b] }
    }

    pub fn from_point_dim(a: &Point2<F>, d: &Vector2<F>) -> Rect<F> {
	Rect { p: [*a, *a + *d] }
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
