//! SVG-related utilites
use itertools::Itertools;
use nalgebra as na;
use std::fmt::Display;
use svg::node::element::{Group, Polyline, Polygon, Line};

pub fn line<F: na::Scalar>(x1: F, y1: F, x2: F, y2: F) -> Line 
    where svg::node::Value: std::convert::From<F>
{
    Line::new().set("x1", x1).set("y1", y1).set("x2", x2).set("y2", y2)
}

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
            points[curr..curr + num_points]
                .iter()
                .map(|p| format!("{},{}", p[0], p[1]))
                .join(" "),
        );
        group = group.add(pl);

        curr += num_points - 1;
    }
    group
}

/// Create one or more polyine-nodes from a list of points, capping
/// the number of points per line as appropriate.
pub fn polygon_to_node<F: na::Scalar + Display>(
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
        let pl = Polygon::new().set(
            "points",
            points[curr..curr + num_points]
                .iter()
                .map(|p| format!("{},{}", p[0], p[1]))
                .join(" "),
        );
        group = group.add(pl);

        curr += num_points - 1;
    }
    group
}
