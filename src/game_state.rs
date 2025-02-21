use macroquad::math::IVec2;

use crate::styles::Styles;

pub struct GameState {
    pub styles: Styles,
    pub mouse_pos: (f32, f32),
    pub tile_highlighted: IVec2,
}

impl GameState {
    pub fn new() -> Self {
        let styles = Styles::new();
        let mouse_pos = (0.0, 0.0);
        let tile_highlighted = IVec2::ZERO;

        Self {
            styles,
            mouse_pos,
            tile_highlighted,
        }
    }
}
