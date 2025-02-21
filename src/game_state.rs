use macroquad::{
    camera::{set_camera, Camera2D},
    math::{f32, IVec2, Rect},
    texture::{load_texture, set_default_filter_mode, FilterMode, Texture2D},
};

use crate::styles::Styles;

pub struct GameState {
    pub styles: Styles,

    pub camera: Camera2D,
    pub mouse_pos: f32::Vec2,
    pub tile_highlighted: IVec2,

    pub player_tile: IVec2,

    pub texture_player: Texture2D,
}

impl GameState {
    pub async fn new() -> Self {
        GameState::configure();

        let styles = Styles::new();

        let camera = GameState::get_camera();
        let mouse_pos = f32::Vec2::ZERO;
        let tile_highlighted = IVec2::ZERO;

        let player_tile = IVec2::ZERO;

        let texture_player = load_texture("assets/el_bueno.png").await.unwrap();

        Self {
            styles,

            camera,
            mouse_pos,
            tile_highlighted,

            player_tile,

            texture_player,
        }
    }

    fn configure() {
        set_default_filter_mode(FilterMode::Nearest);
    }

    fn get_camera() -> Camera2D {
        let camera_rect = Rect {
            x: 0.0,
            y: 0.0,
            w: 512.0,
            h: 288.0,
        };

        let camera_target = f32::vec2(
            camera_rect.x + camera_rect.w / 2.,
            camera_rect.y + camera_rect.h / 2.,
        );
        let camera_zoom = f32::vec2(1. / camera_rect.w * 2., 1. / camera_rect.h * 2.);

        let camera = Camera2D {
            target: camera_target,
            zoom: camera_zoom,
            offset: f32::Vec2::ZERO,
            rotation: 0.,

            render_target: None,
            viewport: None,
        };

        set_camera(&camera);

        camera
    }
}
