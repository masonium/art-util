//! 2D dual contour implementation
use crate::common::*;
use crate::math::types::{Rect, Scalar};
use slotmap::{new_key_type, DenseSlotMap};
use std::convert::TryInto;
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Hermite<F: Scalar> {
    p: Point2<F>,
    n: Vector2<F>
}


#[derive(Clone, Copy, Debug, Hash)]
pub enum CellClass {
    Positive,
    Negative,
    Mixed
}


/// Quadtree order:
/// -------2--------
/// |     +y       |
/// |  3   |   2   |
/// |      |       |
/// 3-x---------+x 1
/// |      |       |
/// |  0   |   1   |
/// |     -y       |
/// -------0--------

/// Quadtree Leaf node.
///
/// Caches function evaluation results at each corner, and contains
/// hermite data for intersections on mixed-sign edges.
#[derive(Clone, Debug)]
struct QtLeaf<F: Scalar> {
    /// Geometry of the leaf node.
    /// Can be inferred, but cached here for speed.
    geom: Rect<F>,

    /// Vertex sign values
    vertex_eval: [F; 4],

    /// Intersection values
    intersections: Vec<Hermite<F>>
}

impl<F: Scalar> QtLeaf<F> {
    /// Return true iff this leaf represents a homogenous-sign region.
    fn is_homogenous(&self) -> bool {
	self.vertex_eval.iter().map(|x| x.signum()).all_equal()
    }

    fn cell_class(&self) -> CellClass {
	if self.vertex_eval.iter().map(|x| x.signum()).all_equal() {
	    if self.vertex_eval[0] > F::zero() {
		CellClass::Positive
	    } else {
		CellClass::Negative
	    }
	} else {
	    CellClass::Mixed
	}
    }
}

new_key_type! { struct QtNodeKey; }

struct QtInterior<F: Scalar> {
    geom: Rect<F>,

    /// Leaf nodes
    children: [QtNodeKey; 4]
}

impl<F: Scalar> QtInterior<F> {
    fn is_homogenous(&self, nm: &QtNodeMap<F>) -> bool {
	self.children.iter().map(|key| nm.get(*key).unwrap()).all(|m| m.is_homogenous(nm))
    }
    /// Return references to nodes
    fn nodes<'a, 't: 'a, 's: 'a>(&'s self, nm: &'t QtNodeMap<F>)
				    -> impl Iterator<Item=&'a QtNode<F>> {
	self.children.iter().map(move |key| nm.get(*key).unwrap())
    }
}

enum QtNode<F: Scalar> {
    Interior(QtInterior<F>),
    Leaf(QtLeaf<F>)
}

type QtNodeMap<F> = DenseSlotMap<QtNodeKey, QtNode<F>>;

impl<F: Scalar> QtNode<F> {
    /// Create a node using the provided function.
    fn build_from_fn<T>(f: &T, bb: &Rect<F>, max_depth: usize,
		     corner_vals: &[F; 4], node_map: &mut QtNodeMap<F>) -> QtNode<F>
    where T: Fn(Point2<F>) -> F {
	// leaf-node-map
	if max_depth == 0 {
	    // create a leaf node, with predetermined
	    let leaf = QtLeaf { geom: *bb, vertex_eval: *corner_vals,
				intersections: Vec::new() };
	    return QtNode::Leaf(leaf);
	}

	// evalaute the remaining edge midpoints and center.
	let mp = bb.midpoints();
	let c = bb.center();

	let mid = [f(mp[0]), f(mp[1]), f(mp[2]), f(mp[3])];
	let c_eval = f(c);

	let cv = &corner_vals;
	let evals = [[cv[0], mid[0], c_eval, mid[3]],
		     [mid[0], cv[1], mid[1], c_eval],
		     [c_eval, mid[1], cv[2], mid[2]],
		     [mid[3], c_eval, mid[2], cv[3]]];
	let corners = bb.corners();
	let rects = [Rect::from_points(&corners[0], &c),
		     Rect::from_points(&mp[0], &mp[1]),
		     Rect::from_points(&c, &corners[2]),
		     Rect::from_points(&mp[3], &mp[2])];

	let mut keys = Vec::with_capacity(4);
	for i in 0..4 {
	    let node = Self::build_from_fn(f, &rects[i], max_depth - 1,
					   &evals[i], node_map);
	    keys.push(node_map.insert(node));
	}

	QtNode::Interior(QtInterior { children: keys.try_into().unwrap(), geom: *bb })
    }

    /// Return a collapsed version of this node, recursive collapsing
    /// internal nodes. Homogenous interior nodes are collapsed into
    /// leaf nodes.
    fn collapse(self, nm: &mut QtNodeMap<F>) -> QtNode<F> {
	match self {
	    leaf @ QtNode::Leaf(_) => leaf,
	    QtNode::Interior(inte) => {
		let nodes: Vec<_> = inte.children.iter()
		    .filter_map(|key| nm.remove(*key))
		    .collect();

		let collapsed_nodes: Vec<_> = nodes.into_iter().map(|n| n.collapse(nm)).collect();

		if collapsed_nodes.iter().all(|n| n.is_leaf() && n.is_homogenous(nm)) {
		    // construct a new leaf node
		    let nodes = collapsed_nodes;
		    let leaves: Vec<QtLeaf<F>> = nodes.into_iter().filter_map(|x| x.into_leaf()).collect();
		    let vals: Vec<_> = leaves.iter().enumerate().map(|(i, leaf)| leaf.vertex_eval[i]).collect();
		    QtNode::Leaf(QtLeaf { geom: inte.geom, vertex_eval: vals.try_into().unwrap(), intersections: Vec::new() })
		} else {
		    // create a new interior node from the collapsed nodes
		    let collapsed_keys: Vec<_> = collapsed_nodes.into_iter().map(|n| nm.insert(n)).collect();
		    QtNode::Interior( QtInterior { geom: inte.geom,
						   children: collapsed_keys.try_into().unwrap()
		    })
		}
	    }
	}
    }

    fn leaf_rects(&self, nm: &QtNodeMap<F>, r: &mut Vec<(Rect<F>, CellClass)>) {
	match self {
	    QtNode::Interior(inte) => {
		inte.nodes(nm).for_each(|n| n.leaf_rects(nm, r))
	    },
	    QtNode::Leaf(leaf) => {
		r.push((leaf.geom, leaf.cell_class()))
	    }
	}
    }

    fn is_valid(&self, nm: &QtNodeMap<F>) -> bool {
	if let QtNode::Interior(ref inte) = self {
	    inte.children.iter().all( |k| nm.contains_key(*k) ) &&
		inte.children.iter().filter_map( |k| nm.get(*k) ).all(|n| n.is_valid(nm))
	} else {
	    true
	}

    }

    /// Return true iff the node represents a homogenous region.
    pub fn is_homogenous(&self, nm: &QtNodeMap<F>) -> bool {
	match self {
	    QtNode::Leaf(leaf) => leaf.is_homogenous(),
	    QtNode::Interior(inte) => inte.is_homogenous(nm)
	}
    }

    /// Deconstruct and return the inner leaf, if it exists.
    fn into_leaf(self) -> Option<QtLeaf<F>> {
	match self {
	    QtNode::Leaf(leaf) => Some(leaf),
	    _ => None
	}
    }

    /// Return true iff this represents a leaf.
    fn is_leaf(&self) -> bool {
	match self {
	    &QtNode::Leaf(_) => true,
	    _ => false
	}
    }

    /// Return the number of leaves in the subtree rooted at this
    /// node, including self.
    fn count_leaves(&self, nm: &QtNodeMap<F>) -> usize {
	match self {
	    QtNode::Leaf(_) => 1,
	    QtNode::Interior(inte) =>
		inte.children.iter().filter_map(|key| nm.get(*key)).map(|n| n.count_leaves(nm)).sum()
	}
    }
}

pub struct QuadTree<F: Scalar> {
    node_data: QtNodeMap<F>,
    root_node: QtNodeKey
}

impl<F: Scalar> QuadTree<F> {
    /// Create a quad tree from an implicit function and a bounding
    /// rectangle. A maximum number of level is specified for the
    /// depth of the quadtree.
    pub fn build_from_fn<T>(f: &T, bb: &Rect<F>, max_depth: usize) -> QuadTree<F>
    where T: Fn(Point2<F>) -> F{
	// Evaluate at the corners to seed the calculation.
	let corners = bb.corners();
	let corner_evals: [F; 4] = [f(corners[0]), f(corners[1]), f(corners[2]), f(corners[3])];
	let mut node_map = DenseSlotMap::with_key();
	let root_node = QtNode::build_from_fn(f, bb, max_depth, &corner_evals, &mut node_map);
	let root_key = node_map.insert(root_node);

	QuadTree { node_data: node_map, root_node: root_key }
    }

    /// Collapse the quad-tree to minimize homogenous leaves.
    pub fn collapse(&mut self) {
	let root = self.node_data.remove(self.root_node).unwrap();
	let new_root = root.collapse(&mut self.node_data);
	self.root_node = self.node_data.insert(new_root);
    }

    pub fn count_leaves(&self) -> usize {
	self.root().count_leaves(&self.node_data)
    }

    /// Shortcut to return the root node through the node_data map.
    fn root(&self) -> &QtNode<F> {
	self.node_data.get(self.root_node).unwrap()
    }

    /// Test-only method.
    /// Validate all keys.
    pub fn is_valid(&self) -> bool {
	self.root().is_valid(&self.node_data)
    }

    pub fn leaf_rects(&self) -> Vec<(Rect<F>, CellClass)> {
	let mut v = Vec::with_capacity(self.count_leaves());
	self.root().leaf_rects(&self.node_data, &mut v);
	v
    }
}
