use bitflags::bitflags;

use crate::{backend::{Backend, Vector2}, layout::Layout, style::Style, theme::{Breadcrumbs, StyleDelegate, Theme}, tree::{LayoutTree, TreeAction}};

bitflags! {
    /// This bitmask defines whether a node contains a point in its boundaries.
    ///
    /// To allow this to default to [`Self::Opaque`] while being
    /// [zero-initialized](`std::mem::MaybeUninit::zeroed`), each bit is inverted;
    /// i.e. `0` means *yes, in bounds* and `1` means, *no, not in bounds*.
    ///
    /// [`HitPassthrough`] is used as a return value of [`Node::in_bounds`]. For most use-cases,
    /// [`Self::Opaque`] and [`Self::Passthrough`] are the most appropriate, specifying that the point in
    /// question is, or is not, in the node's bounds. This defines the way nodes interact with
    /// mouse, touchscreen or other hover events ([`crate::io::hover`]).
    ///
    /// The node is not normally responsible for the bounds of its children. The node's should
    /// only specify its own bounds, so neither a `Passthrough` or `Opaque` answer prevent children nodes
    /// from overriding the answer.
    ///
    /// Despite the above, it is sometimes desirable to keep children from occupying space, for
    /// example to hijack and control mouse input. To specify that children nodes *cannot* be in
    /// bounds, use [`Self::PassthroughBranch`] (to indicate none of the nodes include the point) or
    /// [`Self::PassthroughChildren`] (the node captures all events, including of its children).
    ///
    /// # See Also
    ///
    /// - [`Node::in_bounds`] for a function returning this value.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct HitPassthrough: u8 {
        /// The point is in bounds of this node.
        const Opaque              = 0;

        /// The point is *not* in bounds of this node.
        const Passthrough         = 1;

        /// The point is in bounds, but not in the bounds of any of the children nodes.
        const PassthroughChildren = 2;

        /// Indicates that the point is *not* in bounds of any of the nodes in the branch; neither
        /// of self, nor any of the children nodes.
        const PassthroughBranch   = 3;
    }
}

impl HitPassthrough {
    /// # Returns
    ///
    /// True if the queried point can be found in the node itself.
    pub const fn in_self(self) -> bool {
        !self.contains(Self::Passthrough)
    }

    /// # Returns
    ///
    /// True if the queried point may or may not be found in the children of the node.
    /// A false value indicates that the point will not be in any of the children nodes,
    /// and that children nodes should not be tested.
    pub const fn in_children(self) -> bool {
        !self.contains(Self::PassthroughChildren)
    }

    /// Create a value that combines the restrictions of both masks. It can be said that either
    /// of the masks acts as a "filter", hence the name.
    ///
    /// For example, combining [`Self::Opaque`] with [`Self::Passthrough`] returns [`Self::Passthrough`].
    /// Combining [`Self::Passthrough`] with [`Self::PassthroughChildren`] returns [`Self::PassthroughBranch`].
    ///
    /// # Params
    ///
    /// - `other`: Mask to combine with.
    ///
    /// # Returns
    ///
    /// A mask with `in_self == false` if false for either of the masks,
    /// and similarly `in_children == false` if false for either of the values.
    pub const fn filter(self, other: Self) -> Self {
        self.union(other)
    }

    /// Set the node's opacity filter. This can be used as a node property - an opacity mask
    /// can be passed to a node builder.
    ///
    /// # Params
    ///
    /// - `node`: Node to change.
    pub fn apply<B: Backend>(self, node: &mut Node<B>) {
        node.data.hit_passthrough = self;
    }
}

const _: () = assert!(matches!(HitPassthrough::empty(), HitPassthrough::Opaque));
const _: () = assert!(!HitPassthrough::Passthrough.in_self());
const _: () = assert!( HitPassthrough::Passthrough.in_children());
const _: () = assert!( HitPassthrough::Opaque.in_self());
const _: () = assert!( HitPassthrough::Opaque.in_children());
const _: () = assert!(!HitPassthrough::PassthroughBranch.in_self());
const _: () = assert!(!HitPassthrough::PassthroughBranch.in_children());
const _: () = assert!( HitPassthrough::PassthroughChildren.in_self());
const _: () = assert!(!HitPassthrough::PassthroughChildren.in_children());
const _: () = assert!(matches!(HitPassthrough::Opaque.filter(HitPassthrough::Passthrough), HitPassthrough::Passthrough));
const _: () = assert!(matches!(HitPassthrough::Passthrough.filter(HitPassthrough::PassthroughChildren), HitPassthrough::PassthroughBranch));

/// Represents a UI node.
pub struct NodeData<B: Backend> {
    /// Tree data for the node. Note: requires at least one draw before this will work.
    pub tree: Option<Box<LayoutTree<B>>>,

    /// Layout for this node.
    pub layout: Layout,

    /// Breadcrumbs assigned and applicable to this node. Loaded every resize and every draw.
    pub breadcrumbs: Breadcrumbs<B>,

    /// Filter to apply to every result of `in_bounds`, controlling how the node reacts to
    /// some events, such as mouse click or a finger touch.
    ///
    /// By changing this to [`HitPassthrough::Passthrough`], this can be used to prevent a node from accepting
    /// hover input, making it "invisible". A value of [`HitPassthrough::PassthroughBranch`] will disable the
    /// whole branch, including its children. [`HitPassthrough::PassthroughChildren`] will disable input.
    ///
    /// The default value allows all events.
    pub hit_passthrough: HitPassthrough,

    /// Minimum size of the node.
    pub(crate) min_size: Vector2,

    /// If true, this node must update its size.
    is_resize_pending: bool,

    /// If true, this node is hidden and won't be rendered.
    is_hidden: bool,

    /// If true, this node is currently hovered.
    is_hovered: bool,

    /// If true, this node is currently disabled.
    is_disabled: bool,
    /// Check if this node is disabled, or has inherited the status.
    is_disabled_inherited: bool,

    /// If true, this node will be removed from the tree on the next draw.
    to_remove: bool,

    /// Theme of this node.
    theme: Theme<B>,
    /// True if the theme has been assigned explicitly by a direct assignment. If false, the node will instead
    /// inherit themes from the parent.
    ///
    /// This can be set to false to reset the theme.
    pub is_theme_explicit: bool,

    /// Cached style for this node.
    style: Style<B>,

    /// Attached styling delegates.
    style_delegates: Vec<StyleDelegate<B>>,

    /// Actions queued for this node; only used for queueing actions before the first `resize`; afterwards, all
    /// actions are queued directly into the tree.
    ///
    /// Queues into `TreeContext`.
    queued_actions: Vec<TreeAction<B>>,
}

impl<B: Backend> NodeData<B> {
    /// Construct a new node.
    ///
    /// The typical approach to constructing new nodes is via [`crate::utils::simple_constructor`]. A node component would
    /// provide an alias pointing to the `simple_constructor` instance, which can then be used as a factory function. For
    /// example, [`Label`] provides the `label` simple_constructor. Using these has increased convenience by making it
    /// possible to specify special properties while constructing the node.
    ///
    /// # See Also
    /// - [`crate::utils::simple_constructor`]
    pub const fn new() -> Self {
        Self {
            tree: None,
            layout: todo!(),
            breadcrumbs: todo!(),
            hit_passthrough: todo!(),
            min_size: Vector2::default(),
            is_resize_pending: true,
            is_hidden: false,
            is_hovered: false,
            is_disabled: false,
            is_disabled_inherited: false,
            to_remove: false,
            theme: Theme::new(),
            is_theme_explicit: false,
            style: todo!(),
            style_delegates: Vec::new(),
            queued_actions: Vec::new(),
        }
    }

    /// Check if the node is hidden.
    #[inline]
    pub const fn is_hidden(&self) -> bool {
        self.is_hidden || self.to_remove
    }

    /// Set the visibility
    #[inline]
    pub fn set_hidden(&mut self, value: bool) {
        // If changed, trigger resize
        if self.is_hidden != value { self.update_size(); }

        self.is_hidden = value;
    }

    /// The theme defines how the node will appear to the user.
    ///
    /// Themes affect the node and its children, and can respond to changes in state,
    /// like values changing or user interaction.
    ///
    /// If no theme has been set, a default one will be provided and used automatically.
    ///
    /// See [`Theme`] for more information.
    ///
    /// # Returns
    ///
    /// Currently active theme.
    #[inline]
    pub fn theme(&self) -> &Theme<B> {
        &self.theme
    }

    /// Set the theme.
    #[inline]
    pub fn set_theme(&mut self, value: Theme<B>) {
        self.theme = value;
        self.is_theme_explicit = true;
        self.update_size();
    }

    /// Nodes automatically inherit theme from their parent, and the root node implicitly inherits the default theme.
    /// An explicitly-set theme will override any inherited themes recursively, stopping at nodes that also have themes
    /// set explicitly.
    ///
    /// # Params
    ///
    /// - `value`: Theme to inherit.
    ///
    /// # See Also
    /// - [`Self::theme`]
    #[inline]
    pub fn inherit_theme(&mut self, value: Theme<B>) {
        // Do not override explicitly-set themes
        if self.is_theme_explicit { return; }

        self.theme = value;
        self.update_size();
    }

    /// Clear the currently assigned theme
    #[inline]
    pub fn reset_theme(&mut self) {
        self.theme = Theme::new();
        self.is_theme_explicit = false;
        self.update_size();
    }

    /// Recalculate the window size before next draw.
    #[inline]
    pub fn update_size(&mut self) {
        if let Some(tree) = &mut self.tree {
            tree.root.borrow_mut().data.is_resize_pending = true;
        }
        // Tree might be None - if so, the node will be resized regardless
    }
}

pub type NodeType = std::mem::Discriminant<NodeVariant>;

pub enum NodeVariant {
    // todo
}

impl NodeVariant {
    #[inline]
    #[must_use]
    pub const fn node_type(&self) -> NodeType {
        std::mem::discriminant(self)
    }
}

pub struct Node<B: Backend> {
    pub data: NodeData<B>,
    pub variant: NodeVariant,
}
