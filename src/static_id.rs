/// Unique ID generated from a symbol.
///
/// See `staticID` for generating static IDs.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StaticID {
    /// The ID.
    id: usize,
}
