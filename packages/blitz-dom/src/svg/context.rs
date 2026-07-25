use kurbo::{Affine, BezPath, Rect, Size};
use std::collections::HashMap;
use style::Atom;

/// SVG fragment state, owned by the root <svg> element
pub struct SvgContext {
    pub root: usize,                // DOM node ID of the <svg>
    pub viewport: Size,             // From CSS box of <svg>
    pub viewbox: Option<Rect>,      // From viewBox attr
    pub root_ctm: Affine,           // viewBox → viewport mapping
    pub nodes: Vec<SvgNode>,        // Flat list, render order
    pub id_map: HashMap<Atom, u32>, // url(#id) → index in nodes
}

pub struct SvgNode {
    pub dom_id: usize,
    pub parent: Option<u32>,
    pub ctm: Affine, // User space → viewport space
    pub kind: SvgNodeKind,
    pub bbox: Rect, // Object bounding box, user space
}

pub enum SvgNodeKind {
    Group,
    Shape(BezPath),
    Text,
    Image,
    ForeignObject,
    Use { target_idx: u32 },
    Marker,
}
