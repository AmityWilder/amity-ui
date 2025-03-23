use crate::{backend::{Backend, Vector2}, rope::Rope};

/// Low-level interface for drawing text. Represents a single typeface.
///
/// Unlike the rest of the library, `Typeface` uses screen-space dots directly, instead of fixed-size pixels. Consequently, DPI
/// must be specified manually.
///
/// # See
/// - [`crate::text::Text`] for an interface on a higher level.
pub trait Typeface<B: Backend> {
    /// List glyphs in the typeface.
    fn glyph_count(&self) -> usize;

    /// Get initial pen position.
    fn pen_position(&self) -> Vector2;

    /// Get line height.
    fn line_height(&self) -> i32;

    /// Width of an indent/tab character, in dots.
    /// [`Text`] sets `indent_width` automatically.
    fn indent_width(&self) -> &i32;

    /// Width of an indent/tab character, in dots.
    /// [`Text`] sets `indent_width` automatically.
    fn indent_width_mut(&mut self) -> &mut i32;

    /// Get advance vector for the given glyph. Uses dots, not pixels, as the unit.
    fn advance(&mut self, glyph: char) -> Vector2;

    /// Get curently set DPI.
    fn dpi(&self) -> Vector2;

    /// Set the font size. This should be called at least once before drawing.
    /// [`Text`], if used, sets this automatically.
    ///
    /// Font renderer should cache this and not change the scale unless updated.
    ///
    /// # Params
    ///
    /// - `dpi`:  Horizontal and vertical DPI value, for example (96, 96).
    /// - `size`: Size of the font, in pixels.
    fn set_size(&mut self, dpi: Vector2, size: f32);

    /// Draw a line of text.
    ///
    /// **Note:** This API is unstable and might change over time.
    ///
    /// # Params
    ///
    /// - `target`:       Image to draw to.
    /// - `penPosition`:  Pen position for the beginning of the line. Updated to the pen position at the end of th line.
    /// - `text`:         Text to draw.
    /// - `paletteIndex`: If the image has a palette, this is the index to get colors from.
    fn draw_line(&self, target: &mut B::Image, pen_position: &mut Vector2, text: Rope, palette_index: u8);
}
