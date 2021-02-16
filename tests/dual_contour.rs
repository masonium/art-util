#[cfg(test)]
mod test {
    use na::Point2;
    use nalgebra as na;
    use art_util::Rect;
    use art_util::QuadTree;

    fn create_qt_f32() -> QuadTree<f32> {
	let f = |p: Point2<f32>| { 1.0 - p.coords.norm()  };
	let r: Rect<f32> = Rect::from_points(&Point2::new(-1.5f32, -1.5f32), &Point2::new(1.5f32, 1.5f32));

	QuadTree::build_from_fn(&f, &r, 5)
    }
    fn create_qt_f64() -> QuadTree<f64> {
	let f = |p: Point2<f64>| { 1.0 - p.coords.norm()  };
	let r =  Rect::from_points(&Point2::new(-1.5, -1.5), &Point2::new(1.5, 1.5));

	QuadTree::build_from_fn(&f, &r, 5)
    }

    #[test]
    fn creation_f32() {
	let qt = create_qt_f32();
	assert!(qt.is_valid());
	assert_eq!(qt.count_leaves(), 244);
    }

    #[test]
    fn creation_f64() {
	let qt = create_qt_f64();
	assert!(qt.is_valid());
	assert_eq!(qt.count_leaves(), 244);
    }
}
