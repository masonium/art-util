use nalgebra as na;
//use ncollide2d as nc;

#[derive(Clone, Copy)]
pub enum FrustumPlane {
    Left = 0,
    Right = 1,
    Bottom = 2,
    Top = 3,
    Near = 4,
    Far = 5,
}

/// The collection of planes forming the convex region bounded by a
/// projection matrix or, more generally, a clip matrix.
#[derive(Clone, Debug)]
pub struct Frustum<F: na::RealField> {
    pub planes: [na::Vector4<F>; 6],
}

impl<F: na::RealField> Frustum<F> {
    /// Compute the 6 planes of the frustum defined by a projection matrix
    /// the following order:
    ///
    /// Left, Right, Bottom, Top, Near, Far
    pub fn from_clip_matrix(m: &na::Matrix4<F>) -> Frustum<F> {
	let mt = m.transpose();
	let mut planes = [
		mt.column(3) + mt.column(0),
		mt.column(3) - mt.column(0),
		mt.column(3) + mt.column(1),
		mt.column(3) - mt.column(1),
		mt.column(3) + mt.column(2),
		mt.column(3) - mt.column(2),
	];

	for p in planes.iter_mut() {
	    *p = p.normalize();
	}

	Frustum {
	    planes
	}
    }

    /// Return true iff the point lines within the frustum.
    pub fn is_point_in(&self, v: &na::Point3<F>) -> bool {
	let ext = na::Vector4::new(v[0], v[1], v[2], F::one());
	self.planes.iter().all(|p| {
	    p.dot(&ext) >= F::zero()
	})
    }

    pub fn get_plane(&self, idx: FrustumPlane) -> na::Vector4<F> {
	self.planes[idx as usize]
    }
}
