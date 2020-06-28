//! SVG-related utilites
use itertools::Itertools;
use nalgebra as na;
use std::fmt::Display;
use svg::node::element::{Group, Polyline};

/// Create one or more polyine-nodes from a list of points, capping
/// the number of points per line as appropriate.
pub fn polyline_to_node<F: na::Scalar + Display>(
    points: &[na::Point2<F>],
    points_per_pl: usize,
) -> Group {
    let mut group = Group::new();

    let mut curr = 0;
    while curr < points.len() {
        let num_points = std::cmp::min(points_per_pl, points.len() - curr);
	if num_points <= 1 {
	    break;
	}
        // format the points into a polyline
        let pl = Polyline::new().set(
            "points",
            points[curr..curr+num_points]
                .iter()
                .map(|p| format!("{},{}", p[0], p[1]))
                .join(" "),
        );
        group = group.add(pl);

        curr += num_points - 1;
    }
    group
}
