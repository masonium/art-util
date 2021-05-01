//! Types for dual_contour implementation.
use crate::common::*;

#[derive(Clone, Copy, Debug)]
pub struct HermiteData<F: Scalar> {
    pub p: Point2<F>,
    pub n: Vector2<F>
}
