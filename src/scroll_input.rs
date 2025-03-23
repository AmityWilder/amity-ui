use std::{cell::RefCell, rc::Weak};

use crate::{backend::Backend, canvas::CanvasIO, hover::HoverIO};

///
pub struct ScrollInput<B: Backend> {

    // TODO Hiding a scrollbar makes it completely unusable, since it cannot scan the viewport. Perhaps override
    // `isHidden` to virtually hide the scrollbar, and keep it always "visible" as such?

    pub canvas_io: Box<dyn CanvasIO<B>>,

    /// If true, the scrollbar will be horizontal.
    pub is_horizontal: bool,

    /// Amount of pixels the page is scrolled down.
    pub position: f32,

    /// Available space to scroll.
    ///
    /// Note: visible box size, and therefore scrollbar handle length, are determined from the space occupied by the
    /// scrollbar.
    pub available_space: f32,

    /// Width of the scrollbar.
    pub width: f32, // 10

    /// Handle of the scrollbar.
    pub handle: Box<ScrollInputHandle<B>>,

    /// True if the scrollbar is pressed.
    pub(crate) is_pressed: bool,

    /// If true, the inner part of the scrollbar is hovered.
    pub(crate) inner_hovered: bool,

    /// Page length as determined in resizeImpl.
    pub(crate) page_length: f64,

    /// Length of the scrollbar as determined in drawImpl.
    pub(crate) length: f64,
}

impl<B: Backend> ScrollInput<B> {
    /// Mouse scroll speed; Pixels per event in Scrollable.
    /// Only applies to legacy backend-based I/O.
    pub const SCROLL_SPEED: f32 = 60.0;

    /// Keyboard/gamepad scroll speed in pixels per event.
    pub const ACTION_SCROLL_SPEED: f32 = 60.0;
}

// : Node, FluidHoverable, Hoverable
pub struct ScrollInputHandle<B: Backend> {
    pub hover_io: Box<dyn HoverIO<B>>,
    pub canvas_io: Box<dyn CanvasIO<B>>,

    pub parent: Weak<RefCell<ScrollInput<B>>>,

    /// Length of the handle.
    pub(crate) length: f64,

    /// True if the handle was pressed this frame.
    pub(crate) just_pressed: bool,

    /// Position of the mouse when dragging started.
    pub(crate) start_mouse_position: B::Vector2,

    /// Scroll value when dragging started.
    pub(crate) start_scroll_position: f32,

    is_pressed: bool,
}

impl<B: Backend> ScrollInputHandle<B> {
    pub const MINIMUM_LENGTH: u32 = 50;
}
