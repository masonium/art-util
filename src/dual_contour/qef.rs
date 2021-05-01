use crate::common::*;
use ndarray::Array2;
//use ndarray_linalg::QR;
use super::types::HermiteData;

#[derive(Clone, Debug)]
pub struct QEF<F: Scalar> {
    // A | b
    ab_t_ab: Array2<F>,

    // mass point, to minimize distance to
    mass_point_p: Vector2<F>,

    // mass point dimension, for merging
    mass_point_dim: usize
}

impl<F: Scalar> QEF<F> {
    pub fn new(points: &Vec<HermiteData<F>>) -> QEF<F> {
	// Compute the raw a-b matrix.
	let mut ab: Array2<F> = ndarray::Array2::zeros((points.len(), 3));

	let mut mp = Vector2::new(F::zero(), F::zero());
	for (i, p) in points.iter().enumerate() {
	    ab[[i, 0]] = p.n.x;
	    ab[[i, 1]] = p.n.y;
	    ab[[i, 2]] = p.n.dot(&p.p.coords);
	    mp += p.p.coords;
	}

	QEF { ab_t_ab: ab.t().dot(&ab), mass_point_p: mp, mass_point_dim: points.len() }
    }
}
