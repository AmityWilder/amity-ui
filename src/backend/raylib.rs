use raylib::prelude::*;
use std::{path::Path, time::Duration};
use super::{Backend, MouseCursor};

impl From<super::Vector2> for Vector2 {
    fn from(value: super::Vector2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Vector2> for super::Vector2 {
    fn from(value: Vector2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<super::Rectangle> for Rectangle {
    fn from(value: super::Rectangle) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

impl From<Rectangle> for super::Rectangle {
    fn from(value: Rectangle) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

impl From<super::Color> for Color {
    fn from(value: super::Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

impl From<Color> for super::Color {
    fn from(value: Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

pub struct RaylibBackend<'a> {
    pub rl: &'a mut RaylibHandle,
    pub thread: &'a RaylibThread,
    last_mouse_cursor: MouseCursor,
    draw_area: Rectangle,
    tint: Color,
    scale: f32,
}

impl<'a> Backend for RaylibBackend<'a> {
    type MouseButton = MouseButton;
    type KeyboardKey = KeyboardKey;
    type GamepadButton = GamepadButton;
    type GamepadID = i32;
    type Vector2 = Vector2;
    type Rectangle = Rectangle;
    type Texture = Texture2D;
    type Image = Image;
    type Color = Color;

    #[inline]
    fn is_mouse_button_pressed(&self, button: Self::MouseButton) -> bool {
        self.rl.is_mouse_button_pressed(button)
    }

    #[inline]
    fn is_mouse_button_released(&self, button: Self::MouseButton) -> bool {
        self.rl.is_mouse_button_released(button)
    }

    #[inline]
    fn is_mouse_button_down(&self, button: Self::MouseButton) -> bool {
        self.rl.is_mouse_button_down(button)
    }

    #[inline]
    fn is_mouse_button_up(&self, button: Self::MouseButton) -> bool {
        self.rl.is_mouse_button_up(button)
    }

    #[inline]
    fn is_key_pressed(&self, key: Self::KeyboardKey) -> bool {
        self.rl.is_key_pressed(key)
    }

    #[inline]
    fn is_key_released(&self, key: Self::KeyboardKey) -> bool {
        self.rl.is_key_released(key)
    }

    #[inline]
    fn is_key_down(&self, key: Self::KeyboardKey) -> bool {
        self.rl.is_key_down(key)
    }

    #[inline]
    fn is_key_up(&self, key: Self::KeyboardKey) -> bool {
        self.rl.is_key_up(key)
    }

    #[inline]
    fn is_key_repeated(&self, key: Self::KeyboardKey) -> bool {
        self.rl.is_key_pressed_repeat(key)
    }

    #[inline]
    fn input_character(&mut self) -> Option<char> {
        self.rl.get_char_pressed()
    }

    #[inline]
    fn is_gamepad_button_pressed(&self, gamepad: Self::GamepadID, button: Self::GamepadButton) -> bool {
        self.rl.is_gamepad_button_pressed(gamepad, button)
    }

    #[inline]
    fn is_gamepad_button_released(&self, gamepad: Self::GamepadID, button: Self::GamepadButton) -> bool {
        self.rl.is_gamepad_button_released(gamepad, button)
    }

    #[inline]
    fn is_gamepad_button_down(&self, gamepad: Self::GamepadID, button: Self::GamepadButton) -> bool {
        self.rl.is_gamepad_button_down(gamepad, button)
    }

    #[inline]
    fn is_gamepad_button_up(&self, gamepad: Self::GamepadID, button: Self::GamepadButton) -> bool {
        self.rl.is_gamepad_button_up(gamepad, button)
    }

    #[inline]
    fn is_gamepad_button_repeated(&self, _gamepad: Self::GamepadID, _button: Self::GamepadButton) -> bool {
        unimplemented!()
    }

    #[inline]
    fn set_mouse_position(&mut self, value: Self::Vector2) {
        self.rl.set_mouse_position(value);
    }

    #[inline]
    fn mouse_position(&self) -> Self::Vector2 {
        self.rl.get_mouse_position()
    }

    #[inline]
    fn scroll(&self) -> Self::Vector2 {
        self.rl.get_mouse_wheel_move_v().into()
    }

    #[inline]
    fn set_clipboard(&mut self, value: &str) {
        self.rl.set_clipboard_text(value).unwrap()
    }

    #[inline]
    fn clipboard(&self) -> String {
        self.rl.get_clipboard_text().unwrap()
    }

    #[inline]
    fn delta_time(&self) -> Duration {
        Duration::from_secs_f32(self.rl.get_frame_time())
    }

    #[inline]
    fn has_just_resized(&self) -> bool {
        self.rl.is_window_resized()
    }

    #[inline]
    fn set_window_size(&mut self, value: Self::Vector2) {
        unsafe { raylib::ffi::SetWindowSize(value.x as i32, value.y as i32); }
    }

    #[inline]
    fn window_size(&self) -> Self::Vector2 {
        Self::Vector2::new(self.rl.get_render_width() as f32, self.rl.get_render_height() as f32)
    }

    #[inline]
    fn scale(&self) -> f32 {
        self.scale
    }

    #[inline]
    fn set_scale(&mut self, value: f32) {
        self.scale = value;
    }

    #[inline]
    fn dpi(&self) -> Self::Vector2 {
        self.rl.get_window_scale_dpi()
    }

    #[inline]
    fn hidpi_scale(&self) -> Self::Vector2 {
        const FRAC_1_96: f32 = 1.0 / 96.0;
        let dpi = self.dpi();
        Self::Vector2::new(dpi.x * FRAC_1_96, dpi.y * FRAC_1_96)
    }

    #[inline]
    fn set_area(&mut self, rect: Self::Rectangle) {
        todo!("????")
    }

    #[inline]
    fn area(&self) -> Self::Rectangle {
        todo!("????")
    }

    #[inline]
    fn restore_area(&mut self) {
        todo!()
    }

    #[inline]
    fn set_mouse_cursor(&mut self, value: MouseCursor) {
        todo!()
    }

    #[inline]
    fn mouse_cursor(&self) -> MouseCursor {
        todo!()
    }

    #[inline]
    unsafe fn load_texture_from_image(&mut self, image: Self::Image) -> Self::Texture {
        todo!()
    }

    #[inline]
    unsafe fn load_texture(&mut self, filename: &Path) -> Self::Texture {
        todo!()
    }

    #[inline]
    unsafe fn update_texture(&mut self, texture: Self::Texture, image: Self::Image) {
        todo!()
    }

    #[inline]
    unsafe fn unload_texture(&mut self, texture: Self::Texture) {
        todo!()
    }

    #[inline]
    fn set_tint(&mut self, value: Self::Color) {
        todo!()
    }

    #[inline]
    fn tint(&self) -> Self::Color {
        todo!()
    }

    #[inline]
    fn draw_line(start: Self::Vector2, end: Self::Vector2, color: Self::Color) {
        todo!()
    }

    #[inline]
    fn draw_triangle(a: Self::Vector2, b: Self::Vector2, c: Self::Vector2, color: Self::Color) {
        todo!()
    }

    #[inline]
    fn draw_circle(center: Self::Vector2, radius: f32, color: Self::Color) {
        todo!()
    }

    #[inline]
    fn draw_circle_outline(center: Self::Vector2, radius: f32, color: Self::Color) {
        todo!()
    }

    #[inline]
    fn draw_rectangle(rectangle: Self::Rectangle, color: Self::Color) {
        todo!()
    }

    #[inline]
    fn draw_texture(texture: Self::Texture, rectangle: Self::Rectangle, tint: Self::Color) {
        todo!()
    }

    #[inline]
    fn draw_texture_align(texture: Self::Texture, rectangle: Self::Rectangle, tint: Self::Color) {
        todo!()
    }
}
