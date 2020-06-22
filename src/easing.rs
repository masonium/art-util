//! Implement various easing functions via traits
use num_traits::cast::AsPrimitive;
use num_traits::Float;

pub trait Easeable<F: Float> {
    type Output;
    fn lerp(x: F, a: Self, b: Self) -> Self::Output;
}

impl<F: Float, T> Easeable<F> for T where T: std::ops::Mul<F>, <T as std::ops::Mul<F>>::Output: std::ops::Add {
    type Output = <<T as std::ops::Mul<F>>::Output as std::ops::Add>::Output;
    fn lerp(x: F, a: Self, b: Self) -> Self::Output {
	a * (F::one() - x) + b * x
    }
}


/// Perform a linear interpolation from `a` to `b` by `x`, where x is
/// typically in the range [0.0, 1.0].
///
/// Type genericity allows you to use lerp in combination with the
/// easing functions to ease over vectors or other types.
///
/// # Examples
/// ```
/// # use art_util::easing::{lerp, Quad, Easing};
/// use nalgebra as na;
/// use assert_approx_eq::assert_approx_eq;
///
/// let a = na::Vector2::new(0.0, 3.0);
/// let b = na::Vector2::new(3.0, 0.0);
/// let c: na::Vector2<f32> = lerp(Quad::ease_in_u(0.5), &a, &b);
/// assert_approx_eq!(c.x, 0.75);
/// assert_approx_eq!(c.y, 2.25);
/// ```
pub fn lerp<F: Float, T: Easeable<F>>(x: F, a: T, b: T) -> 
    <T as Easeable<F>>::Output
{
    <T as Easeable<F>>::lerp(x, a, b)
}

/// Clamped linear interpolation, where the `x` values is clamped to
/// [0.0, 1.0]. Equivalent to `lerp(clamp(x, 0.0, 1.0), a, b)`
pub fn clerp<F: Float, T: Easeable<F>>(x: F, a: T, b: T) -> 
    <T as Easeable<F>>::Output
{
    let t = num::clamp(x, F::zero(), F::one());
    <T as Easeable<F>>::lerp(t, a, b)
}

/// Perform the inverse lerp on the first argument z.
/// lerp(inv_lerp(x, a, b), a, b) == x
pub fn inv_lerp<F: Float>(z: F, a: F, b: F) -> F {
    (z - a) / (b - a)
}

/// The `Easing` trait represents a function that can be used to
/// transition from `a` to `b`.
pub trait Easing<F: Float + 'static>
where
    f64: AsPrimitive<F>,
{
    /// This method should be defined for all implementeing
    /// structs. It is how we map the unit interval to the output
    /// interval.
    ///
    /// For any sensible easing function, map_unity(0.0) == 0.0 and
    /// map_unity(1.0) == 1.0). Furthermore:
    ///
    /// * For most easing functions, `map_unity`
    /// will be monotonically increasing, but this is not always the
    /// case.
    /// * `map_unity` corresponds directly to the ease-in transformation, so typically
    /// the derivative at 0.0 will be small, if not 0.0.
    fn map_unity(t: F) -> F;

    /// Transform t in [0.0, 1.0] to the range [`begin`, `end`], with
    /// the output values accelerating towards `end` as t increases.
    fn ease_in(t: F, begin: F, end: F) -> F {
        let z = Self::map_unity(t);
        lerp(z, begin, end)
    }

    /// Transform t in [0.0, 1.0] to the range [`begin`, `end`], with
    /// the output values decelerating towards `end` as t increases.
    fn ease_out(t: F, begin: F, end: F) -> F {
        let z = F::one() - Self::map_unity(F::one() - t);
        lerp(z, begin, end)
    }

    fn ease_in_out(t: F, begin: F, end: F) -> F {
        let half = 0.5.as_();
        let two = 2.0.as_();
        let z = half
            * if t < half {
                Self::map_unity(two * t)
            } else {
                F::one() + Self::map_unity(two * (t - half))
            };

        lerp(z, begin, end)
    }

    /// Equivalent to `ease_in(t, 0.0, 1.0)`.
    fn ease_in_u(t: F) -> F {
	Self::ease_in(t, F::zero(), F::one())
    }

    /// Equivalent to `ease_out(t, 0.0, 1.0)`.
    fn ease_out_u(t: F) -> F {
	Self::ease_out(t, F::zero(), F::one())
    }

    /// Equivalent to `ease_in_out(t, 0.0, 1.0)`.
    fn ease_in_out_u(t: F) -> F {
	Self::ease_in_out(t, F::zero(), F::one())
    }
}

macro_rules! impl_easing {
    ($name:ident, |$var:ident| $fn_impl: expr) => {
        pub struct $name;
        impl<F: Float + 'static> Easing<F> for $name
        where
            f64: AsPrimitive<F>,
        {
            fn map_unity($var: F) -> F {
                $fn_impl
            }
        }
    };
}

impl_easing!(Linear, |t| t);
impl_easing!(Quad, |t| t * t);
impl_easing!(Cubic, |t| t * t * t);
impl_easing!(Quintic, |t| t * t * t * t * t);
impl_easing!(Sine, |t| {
    let half_pi = std::f64::consts::FRAC_PI_2.as_();
    F::one() - (t * half_pi).cos()
});
impl_easing!(Circle, |t| {
    F::one() - (F::one() - t * t).sqrt()
});

#[cfg(test)]
mod test {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_lerp() {
	assert_eq!(lerp(0.7, 0.123, 0.489), 0.3792);
    }

    #[test]
    fn test_inv_lerp() {
	assert_eq!(inv_lerp(0.3792, 0.123, 0.489), 0.7);
    }

    #[test]
    fn test_linear_map_unity() {
        assert_eq!(Linear::map_unity(0.2), 0.2);
        assert_eq!(Linear::map_unity(0.8), 0.8);
    }

    #[test]
    fn test_quad_map_unity() {
        assert_approx_eq!(Quad::map_unity(0.2), 0.04);
        assert_approx_eq!(Quad::map_unity(0.8), 0.64);
    }

    #[test]
    fn test_quad_ease_in() {
        assert_approx_eq!(Quad::ease_in(0.5, 1.0, 10.0), 3.25);
        assert_approx_eq!(Quad::ease_in(0.9, 1.0, 10.0), 8.29);
    }

    #[test]
    fn test_quad_ease_out() {
        assert_approx_eq!(Quad::ease_out(0.5, 1.0, 10.0), 7.75);
        assert_approx_eq!(Quad::ease_out(0.9, 1.0, 10.0), 9.91);
    }
}
