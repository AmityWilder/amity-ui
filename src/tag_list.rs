use std::{collections::BTreeSet, num::NonZeroI64};

/// Node parameter assigning a new set of tags to a node.
pub struct TagList(BTreeSet<TagID>);

/// Unique ID of a node tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagID {
    /// Unique ID of the tag.
    id: NonZeroI64,
}
