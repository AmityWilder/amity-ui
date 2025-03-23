pub enum NodeAlign {
    Start,
    Center,
    End,
    Fill,
}

/// Node parameter for setting the node layout.
pub struct Layout {
    /// Fraction of available space this node should occupy in the node direction.
    ///
    /// If set to `0`, the node doesn't have a strict size limit and has size based on content.
    pub expand: u32,

    /// Align the content box to a side of the occupied space.
    pub node_align: [NodeAlign; 2],
}
