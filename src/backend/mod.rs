use std::{path::Path, time::Duration};
pub mod raylib;

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// `Backend` is an interface making it possible to bind graphics to a library other than Raylib.
///
/// The default unit in graphical space is a **pixel** (`px`), here defined as **1/96 of an inch**. This is unless
/// stated otherwise, as in `Texture`.
///
/// **Warning:** Backend API is unstable and functions may be added or removed with no prior warning.
pub trait Backend {
    type MouseButton;
    type KeyboardKey;
    type GamepadButton;
    type GamepadID;
    type Vector2: Into<self::Vector2> + From<self::Vector2>;
    type Rectangle: Into<self::Rectangle> + From<self::Rectangle>;
    type Texture;
    type Image;
    type Color: Into<self::Color> + From<self::Color>;

    /// Get system's double click time.
    #[inline]
    fn double_click_time(&self) -> Duration {
        Duration::from_millis(500)
    }

    /// Check if the given mouse button has just been pressed.
    fn is_mouse_button_pressed(&self, button: Self::MouseButton) -> bool;
    /// Check if the given mouse button has just been released.
    fn is_mouse_button_released(&self, button: Self::MouseButton) -> bool;
    /// Check if the given mouse button is held down.
    fn is_mouse_button_down(&self, button: Self::MouseButton) -> bool;
    /// Check if the given mouse button is up (not held down).
    fn is_mouse_button_up(&self, button: Self::MouseButton) -> bool;

    /// Check if the given keyboard key has just been pressed.
    fn is_key_pressed(&self, key: Self::KeyboardKey) -> bool;
    /// Check if the given keyboard key has just been released.
    fn is_key_released(&self, key: Self::KeyboardKey) -> bool;
    /// Check if the given keyboard key is held down.
    fn is_key_down(&self, key: Self::KeyboardKey) -> bool;
    /// Check if the given keyboard key is up (not held down).
    fn is_key_up(&self, key: Self::KeyboardKey) -> bool;

    /// If true, the given keyboard key has been virtually pressed again, through a long-press.
    fn is_key_repeated(&self, key: Self::KeyboardKey) -> bool;

    /// Get next queued character from user's input. The queue should be cleared every frame.
    /// Return [`None`] if no character was pressed.
    fn input_character(&mut self) -> Option<char>;

    /// Check if the given gamepad button has been pressed on one of the connected gamepads.
    ///
    /// # Returns
    ///
    /// 0 if the event isn't taking place on any controller. \
    /// Otherwise, the number of the controller it is taking place on.
    fn is_gamepad_button_pressed(&self, gamepad: Self::GamepadID, button: Self::GamepadButton) -> bool;

    /// Check if the given gamepad button has been released on one of the connected gamepads.
    ///
    /// # Returns
    ///
    /// 0 if the event isn't taking place on any controller. \
    /// Otherwise, the number of the controller it is taking place on.
    fn is_gamepad_button_released(&self, gamepad: Self::GamepadID, button: Self::GamepadButton) -> bool;

    /// Check if the given gamepad button is held down on one of the connected gamepads.
    ///
    /// # Returns
    ///
    /// 0 if the event isn't taking place on any controller. \
    /// Otherwise, the number of the controller it is taking place on.
    fn is_gamepad_button_down(&self, gamepad: Self::GamepadID, button: Self::GamepadButton) -> bool;

    /// Check if the given gamepad button is up (not held down) on one of the connected gamepads.
    ///
    /// # Returns
    ///
    /// 0 if the event isn't taking place on any controller. \
    /// Otherwise, the number of the controller it is taking place on.
    fn is_gamepad_button_up(&self, gamepad: Self::GamepadID, button: Self::GamepadButton) -> bool;

    /// If true, the given gamepad button has been virtually pressed again, through a long-press.
    ///
    /// Returns: 0 if no controller had a button repeat this frame, or number of the controller.
    fn is_gamepad_button_repeated(&self, gamepad: Self::GamepadID, button: Self::GamepadButton) -> bool;

    /// Set mouse position
    fn set_mouse_position(&mut self, value: Self::Vector2);
    /// Get mouse position
    fn mouse_position(&self) -> Self::Vector2;

    /// Get scroll value on both axes.
    fn scroll(&self) -> Self::Vector2;

    /// Set system clipboard value.
    fn set_clipboard(&mut self, value: &str);
    /// Get system clipboard value.
    fn clipboard(&self) -> String;

    /// Get time elapsed since last frame.
    fn delta_time(&self) -> Duration;

    /// True if the user has just resized the window.
    fn has_just_resized(&self) -> bool;

    /// Set the size of the window.
    fn set_window_size(&mut self, value: Self::Vector2);
    /// Get the size of the window.
    fn window_size(&self) -> Self::Vector2;

    /// Set scale to apply to whatever is drawn next.
    ///
    /// Suggested implementation is to increase return value of `dpi`.
    fn scale(&self) -> f32;

    fn set_scale(&mut self, value: f32);

    /// Get horizontal and vertical DPI of the window.
    fn dpi(&self) -> Self::Vector2;

    /// Get the DPI value for the window as a scale relative to 96 DPI.
    ///
    /// Suggested implementation
    /// ```no_run
    /// let dpi = self.dpi();
    /// Vector2::new(dpi.x / 96.0, dpi.y / 96.0)
    /// ```
    fn hidpi_scale(&self) -> Self::Vector2;

    /// Set area within the window items will be drawn to; any pixel drawn outside will be discarded.
    fn set_area(&mut self, rect: Self::Rectangle);
    /// Area within the window items will be drawn to; any pixel drawn outside will be discarded.
    fn area(&self) -> Self::Rectangle;

    /// Restore the capability to draw anywhere in the window.
    fn restore_area(&mut self);

    /// Set mouse cursor icon.
    fn set_mouse_cursor(&mut self, value: MouseCursor);
    /// Get mouse cursor icon.
    fn mouse_cursor(&self) -> MouseCursor;

    /// Load a texture from memory.
    unsafe fn load_texture_from_image(&mut self, image: Self::Image) -> Self::Texture;
    /// Load a texture from file.
    unsafe fn load_texture(&mut self, filename: &Path) -> Self::Texture;

    /// Update a texture from an image. The texture must be valid and must be of the same size and format as the image.
    unsafe fn update_texture(&mut self, texture: Self::Texture, image: Self::Image);

    /// Destroy a texture created by this backend. Always use `texture.destroy()` to ensure thread safety.
    ///
    /// If the backend's textures are unloaded on drop, this can be implemented as
    /// ```no_run
    /// _ = texture;
    /// ```
    unsafe fn unload_texture(&mut self, texture: Self::Texture);

    /// Set tint for all newly drawn shapes. The input color for every shape should be multiplied by this color.
    fn set_tint(&mut self, value: Self::Color);

    /// Get current tint color.
    fn tint(&self) -> Self::Color;

    /// Draw a line.
    fn draw_line(start: Self::Vector2, end: Self::Vector2, color: Self::Color);

    /// Draw a triangle, consisting of 3 vertices with counter-clockwise winding.
    fn draw_triangle(a: Self::Vector2, b: Self::Vector2, c: Self::Vector2, color: Self::Color);

    /// Draw a circle.
    fn draw_circle(center: Self::Vector2, radius: f32, color: Self::Color);

    /// Draw a circle, but outline only.
    fn draw_circle_outline(center: Self::Vector2, radius: f32, color: Self::Color);

    /// Draw a rectangle.
    fn draw_rectangle(rectangle: Self::Rectangle, color: Self::Color);

    /// Draw a texture.
    fn draw_texture(texture: Self::Texture, rectangle: Self::Rectangle, tint: Self::Color);

    /// Draw a texture, but ensure it aligns with pixel boundaries, recommended for text.
    fn draw_texture_align(texture: Self::Texture, rectangle: Self::Rectangle, tint: Self::Color);

}

pub enum SystemCursors {
    // Default system cursor.
    SystemDefault,
    // No pointer.
    None,
    // Pointer indicating a link or button, typically a pointing hand. 👆
    Pointer,
    // Cross cursor, often indicating selection inside images.
    Crosshair,
    // Vertical beam indicating selectable text.
    Text,
    // Omnidirectional scroll, content can be scrolled in any direction (panned).
    AllScroll,
    // Cursor indicating the content underneath can be resized horizontally.
    ResizeEW,
    // Cursor indicating the content underneath can be resized vertically.
    ResizeNS,
    // Diagonal resize cursor, top-right + bottom-left.
    ResizeNESW,
    // Diagonal resize cursor, top-left + bottom-right.
    ResizeNWSE,
    // Indicates a forbidden action.
    NotAllowed,
}

pub struct MouseCursor {
    /// Use a system-provided cursor.
    pub system: SystemCursors,
    // TODO user-provided cursor image
}

#[allow(non_upper_case_globals)]
impl MouseCursor {
    pub const SystemDefault : Self = Self { system: SystemCursors::SystemDefault };
    pub const None          : Self = Self { system: SystemCursors::None };
    pub const Pointer       : Self = Self { system: SystemCursors::Pointer };
    pub const Crosshair     : Self = Self { system: SystemCursors::Crosshair };
    pub const Text          : Self = Self { system: SystemCursors::Text };
    pub const AllScroll     : Self = Self { system: SystemCursors::AllScroll };
    pub const ResizeEW      : Self = Self { system: SystemCursors::ResizeEW };
    pub const ResizeNS      : Self = Self { system: SystemCursors::ResizeNS };
    pub const ResizeNESW    : Self = Self { system: SystemCursors::ResizeNESW };
    pub const ResizeNWSE    : Self = Self { system: SystemCursors::ResizeNWSE };
    pub const NotAllowed    : Self = Self { system: SystemCursors::NotAllowed };
}
