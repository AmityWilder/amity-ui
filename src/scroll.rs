use crate::{backend::{Backend, Vector2}, hover::HoverIO, node::Node, scroll_input::ScrollInput};

/// Implement scrolling for the given node.
///
/// This only supports scrolling in one axis.
pub struct Scrollable<B: Backend> {
    pub node: Node<B>,

    pub hover_io: Box<dyn HoverIO<B>>,

    /// Scrollbar for the frame. Can be replaced with a customized one.
    pub scroll_bar: ScrollInput<B>,

    /// minSize including the padding.
    padding_box_size: Vector2,
}
