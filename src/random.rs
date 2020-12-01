//! Various random generation utilities.
use nalgebra_glm as glm;
use nalgebra as na;
use na::{Unit, UnitQuaternion};
use glm::{vec3, Vec3};
use rand_distr::{Distribution, StandardNormal};
use rand::Rng;

/// Return a random unit vector.
pub fn random_unit_vector<R: Rng>(rng: &mut R) -> Unit<Vec3> {
    let dist = StandardNormal {};
    Unit::new_normalize(vec3(dist.sample(rng), dist.sample(rng), dist.sample(rng)))
}

/// Return a uniformly random rotation as a unit quaternion.
pub fn random_quat<R: Rng>(rng: &mut R) -> UnitQuaternion<f32> {
    let v = random_unit_vector(rng);
    let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
    UnitQuaternion::from_axis_angle(&v, angle)
}
