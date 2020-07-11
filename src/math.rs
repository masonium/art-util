use nalgebra as na;

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
