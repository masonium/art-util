#[cfg(test)]
mod test {
    use art_util::{implicit_ray_intersect_2d, line_intersect_2d};
    use assert_approx_eq::assert_approx_eq;
    use nalgebra::Point2;

    #[test]
    fn test_line_isect() {
	let is = line_intersect_2d(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0),
				   Point2::new(1.0, 0.0), Point2::new(0.0, 1.0));

	assert_approx_eq!(is.t1().unwrap(), 0.5);
	assert_approx_eq!(is.t2().unwrap(), 0.5);
    }

    #[test]
    fn test_isect_one_edge() {
	let is = implicit_ray_intersect_2d(Point2::new(0.0, 0.0), Point2::new(0.0, 1.0),
					   Point2::new(-0.5, 1.0), Point2::new(0.5, 1.0));

	dbg!(is);
	assert_approx_eq!(is.t1().unwrap(), 1.0);
	assert_approx_eq!(is.t2().unwrap(), 0.5);
    }
}
