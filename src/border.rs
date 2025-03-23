use crate::{backend::{Backend, Rectangle}, style::{Side, SideArray}};

/// Interface for borders
pub trait Border<B: Backend> {
    /// Apply the border, drawing it in the given box.
    fn apply(&self, backend: &mut B, border_box: Rectangle, size: SideArray<f32>);

    /// Get the rectangle for the given side of the border.
    fn side_rect(&self, source: Rectangle, size: SideArray<f32>, side: Side) -> Rectangle {
        use Side::*;
        match side {
            // Left side
            Left => Rectangle::new(
                source.x,
                source.y + size[Top],
                size[Left],
                source.height - size[Top] - size[Bottom],
            ),

            // Right side
            Right => Rectangle::new(
                source.x + source.width - size[Right],
                source.y + size[Top],
                size[Right],
                source.height - size[Top] - size[Bottom],
            ),

            // Top side
            Top => Rectangle::new(
                source.x + size[Left],
                source.y,
                source.width - size[Left] - size[Right],
                size[Top]
            ),

            // Bottom side
            Bottom => Rectangle::new(
                source.x + size[Left],
                source.y + source.height - size[Bottom],
                source.width - size[Left] - size[Right],
                size[Bottom]
            ),
        }
    }
}
