use crate::{backend::{Backend, Color, MouseCursor}, border::Border, theme::Breadcrumbs, typeface::Typeface};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

/// Side array is a static array defining a property separately for each side of a box, for example margin and border
/// size. Order is as follows: `[left, right, top, bottom]`. You can use `Style.Side` to index this array with an enum.
///
/// Because of the default behavior of static arrays, one can set the value for all sides to be equal with a simple
/// assignment: `array = 8`. Additionally, to make it easier to manipulate the box, one may use the `sideX` and `sideY`
/// functions to get a `float[2]` array of the values corresponding to the given axis (which can also be assigned like
/// `array.sideX = 8`) or the `sideLeft`, `sideRight`, `sideTop` and `sideBottom` functions corresponding to the given
/// sides.
pub struct SideArray<T>(pub [T; 4]);

impl<T> std::ops::Index<Side> for SideArray<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Side) -> &Self::Output {
        self.side(index)
    }
}

impl<T> std::ops::IndexMut<Side> for SideArray<T> {
    #[inline]
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        self.side_mut(index)
    }
}

impl<T> SideArray<T> {
    #[inline]
    pub const fn side(&self, side: Side) -> &T {
        &self.0[side as usize]
    }

    #[inline]
    pub const fn side_mut(&mut self, side: Side) -> &mut T {
        &mut self.0[side as usize]
    }
}

/// Contains the style for a node.
pub struct Style<B: Backend> {
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

    /// Breadcrumbs associated with this style. Used to keep track of tree-aware theme selectors, such as
    /// `children`. Does not include breadcrumbs loaded by parent nodes.
    breadcrumbs: Breadcrumbs<B>,
}
