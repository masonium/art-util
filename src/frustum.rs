use nalgebra as na;

pub enum FrustumPlanes {
    Left = 0,
    Right = 1,
    Bottom = 2,
    Top = 3,
    Near = 4,
    Far = 5,
}

pub struct Frustum {
    pub planes: [na::Vector4<f32>; 6],
}

impl Frustum {
    /// Compute the 6 planes of the frustum defined by a projection matrix
    /// the following order:
    ///
    /// Left, Right, Bottom, Top, Near, Far
    pub fn from_clip_matrix(m: &na::Matrix4<f32>) -> Frustum {
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
    pub fn is_point_in(&self, v: na::Vector3<f32>) -> bool {
	let ext = na::Vector4::new(v[0], v[1], v[2], 1.0);
	self.planes.iter().all(|p| {
	    p.dot(&ext) >= 0.0
	})
    }
}
