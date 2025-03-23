use crate::backend::Backend;

/// Default input actions one can listen to.
pub enum FluidInputAction {
    // Basic
    /// Press the input. Used for example to activate buttons.
    Press,
    /// Submit input, eg. finish writing in textInput.
    Submit,
    /// Cancel the input.
    Cancel,
    /// Open context menu.
    ContextMenu,

    // Focus
    /// Focus previous input.
    FocusPrevious,
    /// Focus next input.
    FocusNext,
    /// Focus input on the left.
    FocusLeft,
    /// Focus input on the right.
    FocusRight,
    /// Focus input above.
    FocusUp,
    /// Focus input below.
    FocusDown,

    // Text navigation
    /// Start a new text line, place a line feed.
    BreakLine,
    /// Move to the previous character in text.
    PreviousChar,
    /// Move to the next character in text.
    NextChar,
    /// Move to the previous word in text.
    PreviousWord,
    /// Move to the next word in text.
    NextWord,
    /// Move to the previous line in text.
    PreviousLine,
    /// Move to the next line in text.
    NextLine,
    /// Move to the beginning of this line; Home key.
    ToLineStart,
    /// Move to the end of this line; End key.
    ToLineEnd,
    /// Move to the beginning.
    ToStart,
    /// Move to the end.
    ToEnd,

    // Editing
    /// Erase last character in an input.
    Backspace,
    /// Erase last a word in an input.
    BackspaceWord,
    /// Delete the next character in an input
    DeleteChar,
    /// Delete the next word in an input
    DeleteWord,
    /// Copy selected content.
    Copy,
    /// Cut (copy and delete) selected content.
    Cut,
    /// Paste selected content.
    Paste,
    /// Undo last action.
    Undo,
    /// Redo last action; Reverse "undo".
    Redo,
    /// Insert a tab into a code editor (tab key)
    InsertTab,
    /// Indent current line or selection in a code editor.
    Indent,
    /// Outdent current line or selection in a code editor (shift+tab).
    Outdent,

    // Selection
    /// Select previous character in text.
    SelectPreviousChar,
    /// Select next character in text.
    SelectNextChar,
    /// Select previous word in text.
    SelectPreviousWord,
    /// Select next word in text.
    SelectNextWord,
    /// Select to previous line in text.
    SelectPreviousLine,
    /// Select to next line in text.
    SelectNextLine,
    /// Select all in text.
    SelectAll,
    /// Select from here to line beginning.
    SelectToLineStart,
    /// Select from here to line end.
    SelectToLineEnd,
    /// Select from here to beginning.
    SelectToStart,
    /// Select from here to end.
    SelectToEnd,

    // List navigation
    /// Navigate to the previous list entry.
    EntryPrevious,
    /// Navigate to the next list entry.
    EntryNext,
    /// Navigate up in a tree, eg. in the file picker.
    EntryUp,

    // Scrolling
    /// Scroll left a bit.
    ScrollLeft,
    /// Scroll right a bit.
    ScrollRight,
    /// Scroll up a bit.
    ScrollUp,
    /// Scroll down a bit
    ScrollDown,
    /// Scroll left by a page. Unbound by default.
    PageLeft,
    /// Scroll right by a page. Unbound by default.
    PageRight,
    /// Scroll up by a page.
    PageUp,
    /// Scroll down by a page.
    PageDown,
}

/// ID of an input action.
#[derive(Debug, PartialEq, Eq)]
pub struct InputActionID {
    /// Unique ID of the action.
    pub id: usize,
}

pub enum InputStrokeItem<B: Backend> {
    KeyboardKey(B::KeyboardKey),
    MouseButton(B::MouseButton),
    GamepadButton(B::GamepadButton),
}

/// Represents a key or button input combination.
pub struct InputStroke<B: Backend> {
    pub input: Vec<InputStrokeItem<B>>,
}

impl<B: Backend> IntoIterator for InputStroke<B> {
    type Item = InputStrokeItem<B>;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.input.into_iter()
    }
}

impl<B: Backend> std::ops::Deref for InputStroke<B> {
    type Target = Vec<InputStrokeItem<B>>;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl<B: Backend> std::ops::DerefMut for InputStroke<B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.input
    }
}

/// Binding of an input stroke to an input action.
pub struct InputBinding<B: Backend> {
    pub action: InputActionID,
    pub trigger: InputStrokeItem<B>,
}

/// A layer groups input bindings by common key modifiers.
pub struct InputLayer<B: Backend> {
    pub modifiers: InputStroke<B>,
    pub bindings: Vec<InputBinding<B>>,
}

impl<B: Backend> PartialEq for InputLayer<B> {
    fn eq(&self, _: &Self) -> bool {
        unimplemented!("InputLayer<B> only implements PartialEq to satisfy requirements of Ord");
    }
}

impl<B: Backend> Eq for InputLayer<B> {}

impl<B: Backend> PartialOrd for InputLayer<B> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<B: Backend> Ord for InputLayer<B> {
    /// When sorting ascending, the lowest value is given to the InputLayer with greatest number of bindings
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.modifiers.len().cmp(&self.modifiers.len())
    }
}
