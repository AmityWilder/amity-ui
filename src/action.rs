use crate::{backend::Backend, context::{IO, IOID}, input::InputActionID};


/// Basic input actions necessary for input actions to work.
enum CoreAction {
    /// This input action is fired in response to the `frame` input event.
    Frame,
}

enum Event {
    NoopEvent,
    FrameEvent,
}

/// I/O interface for mapping input events to input actions.
///
/// Input events correspond to direct events from input devices, like keyboard or mouse.
/// The job of [`ActionIO`] is to translate them into more meaningful input actions, which nodes
/// can set up listeners for.
///
/// [`ActionIO`] will work on nodes that are its children. That means that any input handling node must be placed
/// inside as a child will react to these actions. Similarly, nodes representing input devices, also have to be placed
/// as children.
pub trait ActionIO<B: Backend>: IO<B> {
    /// Pass an input event to transform into an input map.
    ///
    /// The [`ActionIO`] system should withhold all input actions until after its node is drawn. This is when
    /// all input handling nodes that the system interacts with, like [`HoverIO`] and [`FocusIO`], have been processed
    /// and are ready to handle the event.
    ///
    /// Once processing has completed, if the event has triggered an action, the system will trigger the callback that
    /// was passed along with the event. Events that were saved in the system should be discarded.
    ///
    /// Note if an event functions as a modifier - for example the "control" key in a "ctrl+c" action - it should not
    /// trigger the callback. In such case, only the last key, the "C" key in the example, will perform the call.
    /// This is to make sure the event is handled by the correct handler, and only once.
    ///
    /// # Params
    /// - `event`:
    ///   Input event the system should save.
    ///
    /// - `number`:
    ///   A number that will be passed as-is into the callback. Can be used to distinguish between
    ///   different action calls without allocating a closure.
    ///
    /// - `callback`:
    ///   Function to call if the event has triggered an input action.
    ///   The ID of the action will be passed as an argument, along with a boolean indicating if it was
    ///   triggered by an inactive, or active event.
    ///   The number passed into the `emit_event` function will be passed as the third argument to this callback.
    ///   The return value of the callback should indicate if the action was handled or not.
    fn emit_event(event: &mut InputEvent, number: i32, callback: dyn FnOnce(&InputActionID, bool, i32) -> bool);
}

/// Uniquely codes a pressed key, button or a gesture, by using an I/O ID and event code map.
/// Each I/O interface can define its own keys and buttons it needs to map. The way it maps
/// codes to buttons is left up to the interface to define, but it usually is with an enum.
pub struct InputEventCode {
    /// ID for the I/O interface representing the input device. The I/O interface defines a code
    /// for each event it may send. This means the I/O ID along with the event code should uniquely identify events.
    ///
    /// An I/O system can create and emit events that belong to another system in order to simulate events
    /// from another device, however this scenario is likely better handled as a separate binding in [`ActionIO`].
    pub io_id: IOID,

    /// Event code identifying the key or button that triggered the event. These codes are defined
    /// by the I/O interface that send them.
    ///
    /// # See Also
    /// - [`KeyboardIO`] for keyboard codes.
    /// - [`MouseIO`] for mouse codes.
    pub event: i32,
}

/// Represents an event coming from an input device, like a pressed key, button or a gesture.
///
/// This only covers events with binary outcomes: the source of event is active, or it is not.
/// Analog sources like joysticks may be translated into input events but they won't be precise.
pub struct InputEvent {
    /// Code uniquely identifying the source of the event, such as a key, button or gesture.
    pub code: InputEventCode,

    /// Set to true if the event should trigger an input action.
    ///
    /// An input event should be emitted every frame the corresponding button or key is held down, but it will
    /// only be "active" for one of the frames. The one active frame determines when input actions that derive
    /// from the event will be fired.
    ///
    /// For a keyboard key, this will be the first frame the key is held (when it is pressed). For a mouse button,
    /// this will be the last frame (when it is released).
    pub is_active: bool,
}

/// This is a base interface for nodes that respond to input actions. While [`ActionIO`] shouldn't interact
/// with nodes directly, input handling systems like [`FocusIO`] or [`HoverIO`] will expect nodes to implement
/// this interface if they support input actions.
pub trait Actionable<B: Backend> {
    /// Determine if the node can currently handle input.
    ///
    /// Blocking input changes behavior of I/O systems responsible for passing the node input data:
    ///
    /// * A blocked node should NOT have input events called. It is illegal to call `action_impl`. Input method
    ///   and device-specific handlers like `hover_impl` and `focus_impl` usually won't be called either.
    ///
    /// * If the input method has a node selection method like focus or hover, nodes that block input should be
    ///   excluded from selection. If a node starts blocking while already selected may continue to be selected.
    ///
    /// # Returns
    ///
    /// True if the node "blocks" input - it cannot accept input events, nor focus. \
    /// False if the node accepts input, and operates like normal.
    fn blocks_input(&self) -> bool;

    /// Handle an input action.
    ///
    /// This method should not be called for nodes for which `blocksInput` is true.
    ///
    /// # Params
    ///
    /// - `io`:
    ///   I/O input handling system to trigger the action, for example [`HoverIO`] or [`FocusIO`].
    ///   May be None.
    ///
    /// - `number`:
    ///   Number assigned by the I/O system. May be used to fetch a resource from the I/O system if it
    ///   supported.
    ///
    /// - `action`:
    ///   ID of the action to handle.
    ///
    /// - `isActive`:
    ///   If true, this is an active action.
    ///   Most event handlers is only interested in active handlers;
    ///   they indicate the event has changed state (just pressed, or just released),
    ///   whereas an inactive action merely means the button or key is down.
    ///
    /// # Returns
    /// True if the action was handled, false if not.
    fn action_impl(&mut self, io: Option<&mut dyn IO<B>>, number: i32, action: &InputActionID, is_active: bool) -> bool;
}
