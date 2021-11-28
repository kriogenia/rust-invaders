mod render;
mod frame;

pub use render::render;
pub use frame::{Frame, Drawable, new_frame};

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;
