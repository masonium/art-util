use crate::common::*;

/// a + (x - a)* (f(b) - f(a)) / (b -a ) == 0
/// -a * (b-a) / (f(b) - f(a)) == (x -a)
/// a * (1 - (b-a) / (f(b) - f(a))) == x

/// Find a root within the specified range.
///
/// If a root exists, the value `a` returned will have one of the
/// following properties:
///
/// a) |f(a)| < epsilon
/// b) there exists some x, |x-a| < delta, s.t f(x) == 0
pub fn find_root<F: Scalar, T: Fn(F) -> F>(
    f: &T,
    domain: &(F, F),
    delta: F,
    epsilon: F,
) -> Option<F> {
    let (mut l, mut r) = if domain.0 < domain.1 {
        (domain.0, domain.1)
    } else {
        (domain.1, domain.0)
    };
    let (mut fl, mut fr) = (f(l), f(r));

    if fl.signum() != fr.signum() {
        return None;
    }

    let delta = delta.abs();
    let epsilon = epsilon.abs();

    while r - l >= delta {
        // Use the secant line and guess that as the next root.
        let mid = l * (F::one() - (r - l) / (fr - fl));

        let fm = f(mid);
        if fm.abs() < epsilon {
            return Some(mid);
        }
        if fl.signum() == fm.signum() {
            l = mid;
            fl = fm;
        } else {
            r = mid;
            fr = fm;
        }
    }

    // Once the bracket is small enough, just return the secant line
    // root.
    Some(l * (F::one() - (r - l) / (fr - fl)))
}
