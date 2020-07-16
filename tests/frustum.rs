#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;
    use na::{Vector3, Vector4};
    use nalgebra as na;
    use nalgebra_glm as glm;
    use art_util::Frustum;

    #[test]
    fn test_frustum_ortho() {
        let frustum = Frustum::from_clip_matrix(&glm::ortho_rh(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0));

        let target_planes: Vec<_> = [
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::new(-1.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::new(0.0, -1.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 1.0),
        ]
        .iter()
        .map(|x| x.normalize())
        .collect();

        for i in 0..6 {
            assert_approx_eq!(frustum.planes[i].normalize().dot(&target_planes[i]), 1.0);
        }
    }

    #[test]
    fn test_frustum_ortho_shift() {
        // If we move the eye 2 units the right, the left and right planes should shift.
        let view = glm::look_at_rh(
            &Vector3::new(2.0, 0.0, 0.0),
            &Vector3::new(2.0, 0.0, -1.0),
            &Vector3::new(0.0, 1.0, 0.0),
        );
        let frustum = Frustum::from_clip_matrix(&(glm::ortho_rh(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0) * view));

        let target_planes: Vec<_> = [
            Vector4::new(1.0, 0.0, 0.0, -1.0),
            Vector4::new(-1.0, 0.0, 0.0, 3.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::new(0.0, -1.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 1.0),
        ]
        .iter()
        .map(|x| x.normalize())
        .collect();

        for i in 0..6 {
            assert_approx_eq!(frustum.planes[i].normalize().dot(&target_planes[i]), 1.0);
        }
    }

    #[test]
    fn test_frustum_perspective() {
	let frustum = Frustum::from_clip_matrix(
            &(glm::perspective_rh(1.0, std::f32::consts::FRAC_PI_2, 0.01, 1.0)),
        );

        let target_planes: Vec<_> = [
            Vector4::new(1.0, 0.0, -1.0, 0.0),
            Vector4::new(-1.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 1.0, -1.0, 0.0),
            Vector4::new(0.0, -1.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, -1.0, -0.01),
            Vector4::new(0.0, 0.0, 1.0, 1.0),
        ]
        .iter()
        .map(|x| x.normalize())
        .collect();

        for i in 0..6 {
            assert_approx_eq!(frustum.planes[i].normalize().dot(&target_planes[i]), 1.0);
        }
    }
}
