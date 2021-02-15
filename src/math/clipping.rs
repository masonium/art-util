use nalgebra as na;
use crate::math::{Rect, Scalar};
use na::{Point2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipResult<F: na::RealField> {
    OutsideVertical,
    OutsideHorizontal,
    Outside,
    Inside(Point2<F>, Point2<F>),
    Partial(Point2<F>, Point2<F>)
}

impl<F: na::RealField> ClipResult<F> {
    /// Return an `Option` representing the line segment
    /// representation of the clip result, if the result is
    /// non-trivial.
    pub fn ok(&self) -> Option<(Point2<F>, Point2<F>)> {
	match self {
	    Self::Inside(a, b) | Self::Partial(a, b) => Some((*a, *b)),
	    _ => None
	}
    }
}

/// Clip a polyline with a rect.
///
/// The input is a polyline (p0, p1, ..., p_n) and the output is a
/// collection of polylines.
pub fn clip_polyline<F: Scalar>(p: &[Point2<F>],
				r: &Rect<F>)
				-> Vec<Vec<Point2<F>>> {

    let mut polylines = vec![];
    let mut curr: Vec<Point2<F>> = vec![];
    for i in 1..p.len() {
	match clip_line(&p[i-1], &p[i], r) {
	    // clear the current line, nothing else to do.
	    ClipResult::OutsideVertical | ClipResult::OutsideHorizontal | ClipResult::Outside => {
		if !curr.is_empty() {
		    polylines.push(std::mem::take(&mut curr));
		    curr.clear();
		}
		continue;
	    },

	    // start or add to the existing polyline
	    ClipResult::Inside(a, b) => {
		if curr.is_empty() {
		    curr.push(a);
		}
		curr.push(b);
	    },

	    ClipResult::Partial(a, b) => {
		// if current is empty, just add a singleton polyline
		if curr.is_empty() {
		    polylines.push(vec![a, b]);
		} else {
		    // the last was inside, so this must end the polyline
		    curr.push(b);
		    polylines.push(std::mem::take(&mut curr));
		    curr.clear();
		}
	    }
	}
    }

    if !curr.is_empty() {
	polylines.push(curr);
    }
    polylines
}

/// Clip a line using a rect.
///
/// Use Liang-Barksy.
pub fn clip_line<F: Scalar>(a0: &Point2<F>, a1: &Point2<F>,
				   r: &Rect<F>) -> ClipResult<F> {
    let zero = F::zero();
    let one = F::one();

    let lp1 = a0 - a1;
    let lp2 = -lp1;
    let lq1 = a0 - r.p[0];
    let lq2 = r.p[1] - a0;

    let mut pos = F::one();
    let mut neg = F::zero();

    if lp1.x == zero && (lq1.x < zero || lq2.x < zero) {
	return ClipResult::OutsideVertical;
    }
    if lp1.y == zero && (lq1.y < zero || lq2.y < zero) {
	return ClipResult::OutsideHorizontal;
    }
    if lp1.x != zero {
	let r1 = lq1.x / lp1.x;
	let r2 = lq2.x / lp2.x;
	if lp1.x < zero {
	    if neg < r1 {
		neg = r1;
	    }
	    if pos > r2 {
		pos = r2
	    }
	} else {
	    if neg < r2 {
		neg = r2;
	    }
	    if pos > r1 {
		pos = r1;
	    }
	}
    }

    if lp1.y != zero {
	let r1 = lq1.y / lp1.y;
	let r2 = lq2.y / lp2.y;
	if lp1.y < zero {
	    if neg < r1 {
		neg = r1;
	    }
	    if pos > r2 {
		pos = r2
	    }
	} else {
	    if neg < r2 {
		neg = r2;
	    }
	    if pos > r1 {
		pos = r1;
	    }
	}
    }

    if neg > pos {
	return ClipResult::Outside;
    }
    if neg == zero && pos == one {
	return ClipResult::Inside(*a0, *a1);
    }

    ClipResult::Partial(a0 + lp2 * neg, a0 + lp2 * pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outside() {
	let tr = Rect { p: [Point2::new(0.0, 0.0), Point2::new(1.0, 2.0)] };
	let p0 = Point2::new(-0.5, 1.0);
	let p1 = Point2::new(-0.0, 3.0);
	assert_eq!(ClipResult::Outside, clip_line(&p0, &p1, &tr));
    }
    #[test]
    fn test_vertical() {
	let tr = Rect { p: [Point2::new(0.0, 0.0), Point2::new(1.0, 2.0)] };
	let p0 = Point2::new(-0.5, 1.0);
	let p1 = Point2::new(-0.5, 3.0);
	assert_eq!(ClipResult::OutsideVertical, clip_line(&p0, &p1, &tr));

	let p0 = Point2::new(1.5, 1.0);
	let p1 = Point2::new(1.5, 3.0);
	assert_eq!(ClipResult::OutsideVertical, clip_line(&p0, &p1, &tr));
    }
    #[test]
    fn test_horizontal() {
	let tr = Rect { p: [Point2::new(0.0, 0.0), Point2::new(1.0, 2.0)] };
	let p0 = Point2::new(0.5, -0.5);
	let p1 = Point2::new(0.0, -0.5);
	assert_eq!(ClipResult::OutsideHorizontal, clip_line(&p0, &p1, &tr));

	let p0 = Point2::new(0.5, 2.5);
	let p1 = Point2::new(0.0, 2.5);
	assert_eq!(ClipResult::OutsideHorizontal, clip_line(&p0, &p1, &tr));
    }

}
