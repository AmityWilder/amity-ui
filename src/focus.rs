use crate::{action::Actionable, backend::Backend};

/// Nodes implementing this interface can be focused by a `FocusIO` system.
pub trait Focusable<B: Backend>: Actionable<B> {
    /// Handle input. Called each frame when focused.
    ///
    /// This method should not be called if `blocksInput` is true.
    ///
    /// Returns:
    ///     True if focus input was handled, false if it was ignored.
    fn focus_impl(&mut self) -> bool;

    /// Set focus to this node.
    ///
    /// Implementation would usually check `blocksInput` and call `focusIO.focus` on self for this to take effect.
    /// A node may override this method to redirect the focus to another node (by calling its `focus()` method),
    /// or ignore the request.
    ///
    /// Focus should do nothing if the node `isDisabled` is true or if
    fn focus(&mut self);

    /// Returns:
    ///     True if this node has focus. Recommended implementation: `return this == focusIO.focus`.
    ///     Proxy nodes, such as `FieldSlot` might choose to return the value of the node they hold.
    fn is_focused(&self) -> bool;
}
