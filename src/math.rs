use nalgebra as na;

pub fn refract_dir(
    incident: &na::Vector2<f32>,
    normal: &na::Vector2<f32>,
    n1: f32,
    n2: f32,
) -> Option<na::Vector2<f32>> {
    let cos_ti = normal.dot(incident) / (normal.norm() * incident.norm());
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
