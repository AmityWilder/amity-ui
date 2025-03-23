use std::{cell::RefCell, collections::LinkedList, rc::{Rc, Weak}};
use crate::{backend::{Backend, Rectangle}, context::TreeContextData, focus::Focusable, input::{InputBinding, InputLayer}, node::Node, scroll::Scrollable, style::SideArray, theme::Breadcrumbs};

pub struct WithPriority<B: Backend> {
    /// Pick priority based on tree distance from the focused node.
    priority: i32,

    /// Square of the distance between this node and the focused node.
    distance2: f32,

    /// The node.
    node: Box<dyn Focusable<B>>,
}

pub struct FocusDirection<B: Backend> {
    /// Available space box of the focused item after last frame.
    pub last_focus_box: Rectangle,

    /// Nodes that may get focus with tab navigation.
    pub prev: Box<dyn Focusable<B>>,
    pub next: Box<dyn Focusable<B>>,

    /// First and last focusable nodes in the tree.
    pub first: Box<dyn Focusable<B>>,
    pub last: Box<dyn Focusable<B>>,

    /// Focusable nodes, by direction from the focused node.
    pub positional: SideArray<WithPriority<B>>,

    /// Focus priority for the currently drawn node.
    ///
    /// Increased until the focused node is found, decremented afterwards. As a result, values will be the highest for
    /// nodes near the focused one. Changes with tree depth rather than individual nodes.
    pub priority: i32,

    /// Value `priority` is summed with on each step. `1` before finding the focused node, `-1` after.
    priority_direction: i32,

    /// Current tree depth.
    depth: u32,
}

/// A class for iterating over the node tree.
pub struct TreeAction<B: Backend> {
    /// Node to descend into; `before_draw` and `after_draw` will only be emitted for this node and its children.
    ///
    /// May be null to enable iteration over the entire tree.
    pub start_node: Node<B>,

    /// If true, this action is complete and no callbacks should be ran.
    ///
    /// Overloads of the same callbacks will still be called for the event that prompted stopping.
    pub to_stop: bool,

    /// Keeps track of the number of times the action has been started or stopped. Every start and every stop
    /// bumps the generation number.
    ///
    /// The generation number is used to determine if the action runner should continue or discontinue the action.
    /// If the number is greater than the one the runner stored at the time it was scheduled, it will stop running.
    /// This means that if an action is restarted, the old run will be unregistered, preventing the action from
    /// running twice at a time.
    ///
    /// Only applies to actions started using `Node::start_action`, introduced in 0.7.2, and not `Node::run_action`.
    pub generation: i32,

    /// Subscriber for events, i.e. `then`
    finished: Box<dyn FnMut()>,

    /// Set to true once the action has descended into `start_node`.
    in_start_node: bool,

    /// Set to true once `before_tree` is called. Set to `false` afterwards.
    in_tree: bool,
}

/// Global data for the layout tree.
pub struct LayoutTree<B: Backend> {
    // Nodes

    /// Root node of the tree.
    pub root: Rc<RefCell<Node<B>>>,

    /// Node the mouse is hovering over if any.
    ///
    /// This is the last - topmost - node in the tree with `is_hovered` set to true.
    pub hover: Weak<RefCell<Node<B>>>,

    /// Currently focused node.
    ///
    /// Changing this value directly is discouraged. Some nodes might not want the focus! Be gentle, call
    /// `Focusable.focus()` instead and let the node set the value on its own.
    pub focus: Box<dyn Focusable<B>>,

    /// Deepest hovered scrollable node.
    pub scroll: Scrollable<B>,

    // Input

    /// Focus direction data.
    pub focus_direction: FocusDirection<B>,

    /// Padding box of the currently focused node. Only available after the node has been drawn.
    ///
    /// See_also: `focusDirection.lastFocusBox`.
    pub focus_box: Rectangle,

    /// Tree actions queued to execute during next draw.
    pub actions: LinkedList<TreeAction<B>>,

    /// Input strokes bound to emit given action signals.
    ///
    /// Input layers have to be sorted.
    pub bound_inputs: Vec<InputLayer<B>>,

    /// Actions that are currently held down.
    pub down_actions: LinkedList<InputBinding<B>>,

    /// Actions that have just triggered.
    pub active_actions: LinkedList<InputBinding<B>>,

    /// Access to core input and output facilities.
    pub backend: B,

    /// True if keyboard input was handled during the last frame; updated after tree rendering has completed.
    pub was_keyboard_handled: bool,

    /// Miscelleanous, technical properties.

    /// Current node drawing depth.
    pub depth: u32,

    /// Current rectangle drawing is limited to.
    pub scissors: Rectangle,

    /// True if the current tree branch is marked as disabled (doesn't take input).
    pub is_branch_disabled: bool,

    /// Current breadcrumbs. These are assigned to any node that is resized or drawn at the time.
    ///
    /// Any node that introduces its own breadcrumbs will push onto this stack, and pop once finished.
    pub breadcrumbs: Breadcrumbs<B>,

    /// Context for the new I/O system.
    pub context: TreeContextData<B>,

    /// Incremented for every `filter_actions` access to prevent nested accesses from breaking previously made ranges.
    action_access_counter: i32,
}
