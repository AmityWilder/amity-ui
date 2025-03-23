use std::{cell::RefCell, rc::Weak};

/// Rope implementation, providing more efficient modification if there's lots of text.
///
/// The `Rope` structure acts as a slice, a view into the rope's contents. If additional text is added to a node stored
/// inside, the change will not be reflected by the rope.
///
/// `rope.init` is guaranteed to be valid and empty.
///
/// See_Also: https://en.wikipedia.org/wiki/Rope_(data_structure)
pub struct Rope {
    /// Content of the rope, if it contains children.
    node: Weak<RefCell<RopeNode>>,

    /// Content of the rope if it's a leaf. Not sliced; to get the text with the slice applied, use `value`.
    ///
    /// This must be a fully valid string. Content may not be split in the middle of a codepoint.
    leaf_text: Box<[u8]>,

    /// Start and length of the rope, in UTF-8 bytes.
    start: usize,
    length: usize,

    /// Depth of the node.
    depth: i32,
}

pub struct RopeNode {
    /// Left child of this node.
    pub left: Rope,

    /// Right child of this node.
    pub right: Rope,
}
