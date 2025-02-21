use macroquad::{
    math::IVec2,
    texture::{load_texture, Texture2D},
};

use crate::styles::Styles;

pub struct GameState {
    pub styles: Styles,

    pub mouse_pos: (f32, f32),
    pub tile_highlighted: IVec2,

    pub player_tile: IVec2,

    pub texture_player: Texture2D,
}

impl GameState {
    pub async fn new() -> Self {
        let styles = Styles::new();

        let mouse_pos = (0.0, 0.0);
        let tile_highlighted = IVec2::ZERO;

        let player_tile = IVec2::ZERO;

        let texture_player = load_texture("assets/el_bueno.png").await.unwrap();

        Self {
            styles,

            mouse_pos,
            tile_highlighted,

            player_tile,

            texture_player,
        }
    }
}
