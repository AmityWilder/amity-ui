use crate::{backend::Backend, context::IO};

/// I/O interface for canvas drawing functionality.
///
/// The canvas should use a coordinate system where (0,0) is the top-left corner. Every increment of 1 is equivalent
/// to the distance of 1/96th of an inch. Consequentially, (96, 96) is 1 inch down and 1 inch right from the top-left
/// corner of the canvas.
///
/// The canvas should allow all inputs and never throw. If there's a defined boundary, the canvas should crop all
/// geometry to fit.
pub trait CanvasIO<B: Backend>: IO<B> {
    // todo
}
