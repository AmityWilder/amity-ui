use crate::{backend::Backend, context::IO};

/// `HoverIO` is an input handler system that reads events off devices with the ability to point at the screen,
/// like mouses, touchpads or pens.
///
/// Most of the time, `HoverIO` systems will pass the events they receive to an `ActionIO` system, and then send
/// these actions to a hovered node.
///
/// Multiple different `HoverIO` instances can coexist in the same tree, allowing for multiple different nodes
/// to be hovered at the same time, as long as they belong in different branches of the node tree. That means
/// two different nodes can be hovered by two different `HoverIO` systems, but a single `HoverIO` system can only
/// hover a single node.
pub trait HoverIO<B: Backend>: IO<B> {
    // todo
}
