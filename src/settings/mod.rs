pub const CELL_WIDTH: u32 = 64;
pub const SCREEN_WIDTH: u32 = CELL_WIDTH * 8;
pub const SCREEN_HEIGHT: u32 = CELL_WIDTH * 8;

pub const PI: f64 = 3.14159265358979323846;
pub const FOV: f64 = PI / 3.0;
pub const HALF_FOV: f64 = FOV / 2.0;
pub const NUM_RAYS: u32 = SCREEN_WIDTH / 2;
pub const DELTA_ANGLE: f64 = FOV / NUM_RAYS as f64;

pub const MAP: [[i32; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 1, 1],
    [1, 0, 0, 0, 1, 0, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];
