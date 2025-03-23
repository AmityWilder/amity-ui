use std::collections::BTreeSet;
use crate::{backend::Backend, static_id::StaticID, tree::TreeAction};

pub struct TreeContext<'a, B: Backend> {
    pub ptr: Option<&'a TreeContextData<B>>,
}

pub struct TreeContextMut<'a, B: Backend> {
    pub ptr: Option<&'a mut TreeContextData<B>>,
}

pub struct TreeContextData<B: Backend> {
    /// Keeps track of currently active I/O systems.
    pub io: TreeIOContext<B>,

    /// Manages and runs tree actions.
    pub actions: TreeActionContext<B>,

    lock_tint: i32,

    tint: B::Color,
}

pub struct IOInstance<B: Backend> {
    pub id: IOID,
    pub io: Box<dyn IO<B>>,
}

impl<B: Backend> PartialEq for IOInstance<B> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<B: Backend> Eq for IOInstance<B> {}

impl<B: Backend> PartialOrd for IOInstance<B> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<B: Backend> Ord for IOInstance<B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

/// Active context for I/O operations. Keeps track of currently active systems for each I/O interface.
///
/// I/O systems are changed by a replace operation. `replace` takes the new I/O systems, but returns the one set
/// previously. This can be used to manage I/Os as a stack:
///
/// ---
/// auto previous = io.replace(id, this);
/// scope (exit) io.replace(id, previous);
/// ---
pub struct TreeIOContext<B: Backend> {
    /// Key-value pairs of active I/O systems. Each pair contains the system and the ID of the interface
    /// it implements. Pairs are sorted by the interface ID.
    active_ios: BTreeSet<Vec<IOInstance<B>>>,
}

pub trait HasContext<B: Backend> {
    /// Returns the current tree context.
    fn tree_context(&self) -> &TreeContext<B>;

    /// Returns the current tree context.
    fn tree_context_mut(&mut self) -> &mut TreeContext<B>;
}

pub trait IO<B: Backend>: HasContext<B> {
    /// Load a resource by reference. This is the same as `Node.load`.
    /// Params:
    ///     resource = Resource to load. It will be updated with identifying information.
    fn load_to(&self); // TODO
}

/// ID for an I/O interface.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct IOID {
    id: StaticID,
}

struct RunningAction<B: Backend> {
    action: TreeAction<B>,
    generation: i32,
}

/// Keeps track of currently active actions.
pub struct TreeActionContext<B: Backend> {
    /// Currently running actions.
    actions: Vec<RunningAction<B>>,

    /// Number of running iterators. Removing tree actions will only happen if there is exactly one
    /// running iterator, as to not break the other ones.
    ///
    /// Multiple iterators may run in case a tree action draws nodes on its own: one iterator triggers
    /// the action, and the drawn node activates another iterator.
    running_iterators: i32,
}
