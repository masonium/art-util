use crate::math::Scalar;
pub use na::{Point2, Vector2};
use nalgebra as na;
use nalgebra_glm::TMat2;
use num_traits::Zero;

pub fn refract_dir(
    incident: na::Vector2<f32>,
    normal: na::Vector2<f32>,
    n1: f32,
    n2: f32,
) -> Option<na::Vector2<f32>> {
    let cos_ti = normal.dot(&incident) / (normal.norm() * incident.norm());
    let n = n1 / n2;
    let sin_tr = n * (1.0 - cos_ti * cos_ti).sqrt();
    // sufficiently close to the critical angle.
    if (sin_tr - 1.0).abs() < 1e-5 {
        return None;
    }

    // refract or reflect, depending on the angle
    let new_dir = if sin_tr > 1.0 {
        // total internal reflection
        incident - 2.0 * incident.dot(&normal) * normal
    } else {
        // refract at the new angle.
        let c1 = cos_ti;
        let c2 = (1.0 - n * n * (1.0 - c1 * c1)).sqrt();
        n * incident + (n * c1 - c2) * normal
    };

    Some(new_dir)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RayInt<T: Scalar> {
    Colinear,
    Parallel,
    Intersection(T, T),
}

fn inside_line_range<T: Scalar>(t: T) -> bool {
    (t.into() >= 1.0e-8) && t.into() <= (1.0 - 1.0e-8)
}

impl<T: Scalar> RayInt<T> {
    /// Return true iff the intersection represents a line-line
    /// intersection, rather than just a ray-ray intersection.
    #[allow(unused)]
    fn is_line_line_isect(&self) -> bool {
        if let Self::Intersection(a, b) = self {
            inside_line_range(*a) && inside_line_range(*b)
        } else {
            false
        }
    }

    /// Return the t1 value if this is an intersection.
    pub fn t1(&self) -> Option<T> {
        if let Self::Intersection(a, _) = self {
            Some(*a)
        } else {
            None
        }
    }

    /// Return the t2 value if this is an intersection.
    pub fn t2(&self) -> Option<T> {
        if let Self::Intersection(_, b) = self {
            Some(*b)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PointTest {
    Inside,
    On,
    Outside,
}

/// Test where the point `p` is in relation to the oriented line defined by `(a, b)`.
pub fn orient_2d<F: Scalar>(p: Point2<F>, a: Point2<F>, b: Point2<F>) -> PointTest {
    let p01 = a - p;
    let p12 = b - a;

    let l01 = p01.norm();
    let l12 = p12.norm();

    let sa = p01.x * p12.y - p01.y * p12.x;
    let f_sa = sa.into();
    let thresh = 1.0e-8 * (l01 * l12).into();
    if f_sa > thresh {
        PointTest::Inside
    } else if -f_sa < -thresh {
        PointTest::Outside
    } else {
        PointTest::On
    }
}

/// Return true iff (p0, p1, p2) form a denegerate triangle.
pub fn is_degen_tri<F: Scalar>(p0: Vector2<F>, p1: Vector2<F>, p2: Vector2<F>) -> bool {
    let p01 = p1 - p0;
    let p12 = p2 - p1;

    let l01 = p01.norm();
    let l12 = p12.norm();

    if l01.into() <= 1.0e-8 || l12.into() <= 1.0e-8 {
        return true;
    }

    let signed_area = p01.x * p12.y - p01.y * p12.x;
    signed_area.abs().into() <= 1.0e-8 * (l01 * l12).into()
}

/// Return the intersection point of two rays, each implicitly defined
/// by two points, assuming any finite t's are valid.
pub fn implicit_ray_intersect_2d<F: Scalar>(
    a0: Point2<F>,
    a1: Point2<F>,
    b0: Point2<F>,
    b1: Point2<F>,
) -> RayInt<F> {
    let da: Vector2<F> = a1 - a0;
    let db = b1.coords - b0.coords;
    let zero = <F as Zero>::zero();

    // The matrix inversion can work even in cases that we would call
    // degenerate, so it's important to check for degenerate cases first.
    if is_degen_tri(a0.coords, a1.coords, b0.coords)
        && is_degen_tri(a0.coords, a1.coords, b1.coords)
    {
        RayInt::Colinear
    } else if is_degen_tri::<F>(Vector2::new(zero, zero), da, db) {
        RayInt::Parallel
    } else {
        let m: TMat2<F> = TMat2::new(da.x, -db.x, da.y, -db.y);
        match m.try_inverse() {
            Some(inv) => {
                let t = inv * (b0 - a0).xy();
                RayInt::Intersection(t.x, t.y)
            }
            None => RayInt::Parallel,
        }
    }
}

/// Return the intersection point along a and b if the lines
/// intersect. Otherwise, return colinear or no intersection as
/// appropriate.
///
/// z-values are ignored.
pub fn line_intersect_2d<F: Scalar>(
    a0: Point2<F>,
    a1: Point2<F>,
    b0: Point2<F>,
    b1: Point2<F>,
) -> RayInt<F> {
    let isect = implicit_ray_intersect_2d(a0, a1, b0, b1);
    match isect {
        RayInt::Intersection(ta, tb) => {
            if inside_line_range(ta) && inside_line_range(tb) {
                RayInt::Intersection(ta, tb)
            } else {
                RayInt::Parallel
            }
        }
        _ => isect,
    }
}
