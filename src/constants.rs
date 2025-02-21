pub const TILE_SIZE_PX: f32 = 32.0;

pub const TILE_SIZE_X: f32 = TILE_SIZE_PX;
pub const TILE_SIZE_Y: f32 = TILE_SIZE_PX;

pub const GRID_W: i32 = 5;
pub const GRID_H: i32 = 3;
pub const GRID_THICKNESS: f32 = 1.0;

pub const SCREEN_W: f32 = TILE_SIZE_X * 16.0;
pub const SCREEN_H: f32 = TILE_SIZE_Y * 9.0;

pub const GRID_SIZE_X: f32 = TILE_SIZE_X * GRID_W as f32;
pub const GRID_SIZE_Y: f32 = TILE_SIZE_Y * GRID_H as f32;

pub const GRID_OFFSET_X: f32 = (SCREEN_W - GRID_SIZE_X) / 2.0;
pub const GRID_OFFSET_Y: f32 = (SCREEN_H - GRID_SIZE_Y) / 2.0;
