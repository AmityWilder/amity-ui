use std::collections::HashMap;
use crate::{backend::{Backend, Color, MouseCursor}, border::Border, node::{Node, NodeType}, tag_list::TagList, typeface::Typeface};

/// Node theme.
pub struct Theme<B: Backend> {
    pub rules: HashMap<NodeType, Vec<Rule<B>>>,
}

impl<B: Backend> Theme<B> {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }
}

pub type StyleDelegate<B> = Box<dyn FnMut(&mut Node<B>) -> Rule<B>>;

/// Rules specify changes that are to be made to the node's style.
pub struct Rule<B: Backend> {
    /// Selector to filter items that should match this rule.
    pub selector: Selector,

    /// Fields affected by this rule and their values.
    pub fields: StyleTemplate<B>,

    /// Callback for updating the style dynamically. May be null.
    pub style_delegate: Option<StyleDelegate<B>>,

    /// Breadcrumbs, if any, assigned to nodes matching this rule.
    pub breadcrumbs: Breadcrumbs<B>,
}

/// Selector is used to pick a node based on its type and specified tags.
pub struct Selector {
    /// Type of the node to match.
    pub node_type: NodeType,

    /// Tags needed by the selector.
    pub tags: TagList,

    /// If true, this selector will reject any match.
    pub reject_all: bool,
}

pub struct Breadcrumbs<B: Backend> {
    /// All rules activated by this instance.
    crumbs: Vec<Vec<Rule<B>>>,

    /// Cached children instances.
    children: Vec<Self>,
}

pub struct StyleTemplate<B: Backend> {
    // Text options

    /// Main typeface to be used for text.
    ///
    /// Changing the typeface requires a resize.
    typeface: Box<dyn Typeface<B>>,

    /// Size of the font in use, in pixels.
    ///
    /// Changing the size requires a resize.
    font_size: f32,

    /// Text color.
    text_color: Color,

    // Background & content

    /// Color of lines belonging to the node, especially important to separators and sliders.
    line_color: Color,

    /// Background color of the node.
    background_color: Color,

    /// Background color for selected text.
    selection_background_color: Color,

    // Spacing

    /// Margin (outer margin) of the node. `[left, right, top, bottom]`.
    ///
    /// Updating margins requires a resize.
    ///
    /// See: `is_side_array`.
    margin: [f32; 4],

    /// Border size, placed between margin and padding. `[left, right, top, bottom]`.
    ///
    /// Updating border requires a resize.
    ///
    /// See: `is_side_array`
    border: [f32; 4],

    /// Padding (inner margin) of the node. `[left, right, top, bottom]`.
    ///
    /// Updating padding requires a resize.
    ///
    /// See: `is_side_array`
    padding: [f32; 4],

    /// Margin/gap between two neighboring elements; for container nodes that support it.
    ///
    /// Updating the gap requires a resize.
    gap: [f32; 2],

    /// Border style to use.
    ///
    /// Updating border requires a resize.
    border_style: Box<dyn Border<B>>,

    // Misc

    /// Apply tint to all node contents, including children.
    tint: Color,

    /// Cursor icon to use while this node is hovered.
    ///
    /// Custom image cursors are not supported yet.
    mouse_cursor: MouseCursor,
}
