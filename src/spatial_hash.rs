//! Quick and dirty 2D spatial hash for quick nearest-point and query selection

use nalgebra::Scalar;
use nalgebra::{Point2, SimdRealField};
use ndarray::prelude::*;
use num::Float;
use num_integer::Roots;
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;

/// SpatialHash2D is a structure for enabling quick nearest-neighbor
/// searches of a set of points. Points can be updated and inserted
/// (though inserting many additional points may diminish the
/// efficiency of the hash).
pub struct SpatialHash2D<F: Scalar + Float + SimdRealField + TryFrom<u16> + Debug> {
    min_range: Point2<F>,
    max_range: Point2<F>,
    sn: F,
    sn_i: u16,
    points: Vec<Point2<F>>,
    grid_accel: Array2<Vec<usize>>,
}

impl<F: Scalar + Float + SimdRealField + TryFrom<u16> + Debug> SpatialHash2D<F> {
    fn sindex(&self, p: &Point2<F>) -> (u16, u16) {
        let rx = (p.x - self.min_range.x) / (self.max_range.x - self.min_range.x);
        let ry = (p.y - self.min_range.y) / (self.max_range.y - self.min_range.y);

        (
            (rx * self.sn).to_u64().unwrap().try_into().unwrap(),
            (ry * self.sn).to_u64().unwrap().try_into().unwrap(),
        )
    }

    pub fn in_range(&self, p: Point2<F>) -> bool {
        self.min_range <= p && p < self.max_range
    }

    pub fn insert(&mut self, p: Point2<F>) {
        let n = self.points.len();
        self.points.push(p);
        let idx = self.sindex(&p);
        self.grid_accel[(idx.0 as usize, idx.1 as usize)].push(n);
    }

    pub fn size(&self) -> usize {
        self.points.len()
    }

    /// Create a spatial hash structure with an intended capacity
    pub fn with_capacity(min_range: Point2<F>, max_range: Point2<F>, n: usize) -> SpatialHash2D<F> {
        let sn = (n.sqrt() + 1) as u16;
        let grid_accel = Array2::default((sn as usize, sn as usize));

        SpatialHash2D {
            min_range,
            max_range,
            sn: sn.try_into().ok().unwrap(),
            sn_i: sn,
            points: vec![],
            grid_accel,
        }
    }

    /// Create a spatial hash structure from an initial set of points.
    pub fn from_points(
        min_range: Point2<F>,
        max_range: Point2<F>,
        v: &[Point2<F>],
    ) -> SpatialHash2D<F> {
        let mut s_hash = Self::with_capacity(min_range, max_range, v.len());

        for p in v {
            s_hash.insert(*p);
        }

        s_hash
    }

    pub fn nearest_neighbor(&self, p: Point2<F>) -> Option<usize> {
        if !self.in_range(p) {
            return None;
        }

        let idx = self.sindex(&p);
        let mut best_p = None;
        let mut best_dist_sq = None;

        for w in 1..=self.sn_i {
            let min_ix: usize = std::cmp::max(idx.0 as isize - w as isize, 0) as usize;
            let max_ix: usize = std::cmp::min(idx.0 as usize + w as usize, self.sn_i as usize);
            let min_iy: usize = std::cmp::max(idx.1 as isize - w as isize, 0) as usize;
            let max_iy: usize = std::cmp::max(idx.0 as usize + w as usize, self.sn_i as usize);

            for ix in min_ix..max_ix {
                for iy in min_iy..max_iy {
                    for pi in &self.grid_accel[[ix, iy]] {
                        let d = self.points[*pi] - p;
                        let d2 = d.dot(&d);
                        if let Some(best_d2) = best_dist_sq {
                            if best_d2 < d2 {
                                continue;
                            }
                        }
                        best_dist_sq = Some(d2);
                        best_p = Some(*pi);
                    }
                }
            }

            if best_p.is_some() {
                return best_p;
            }
        }

        panic!("Failed to find any point.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locate() {
        let points = vec![
            Point2::new(0.5, 0.5),
            Point2::new(1.5, 0.5),
            Point2::new(1.5, 1.5),
        ];
        let shash =
            SpatialHash2D::from_points(Point2::new(0.0, 0.0), Point2::new(2.0, 2.0), &points);

        assert_eq!(shash.nearest_neighbor(Point2::new(0.0, 0.2)), Some(0));
        assert_eq!(shash.nearest_neighbor(Point2::new(-1.0, 0.0)), None);
    }
}
