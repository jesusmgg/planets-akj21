use macroquad::{
    audio::{load_sound, Sound},
    math::{f32, IVec2},
    shapes::draw_rectangle,
    texture::{load_texture, Texture2D},
    window::clear_background,
};

use crate::{constants::*, planet::Planet};
use crate::{styles::Styles, text::draw_scaled_text};

#[derive(Clone)]
pub struct GameState {
    pub styles: Styles,

    pub mouse_pos: f32::Vec2,
    pub is_mouse_in_grid: bool,
    pub tile_highlighted_prev: IVec2,
    pub tile_highlighted: IVec2,

    pub levels: Vec<Level>,
    pub level_active: Option<usize>,
    pub planet_current_index: usize,

    pub score: i32,

    pub sim_step: usize,
    pub sim_step_computed: usize,

    pub texture_explosion_01: Texture2D,
    pub texture_background_01: Texture2D,

    pub sfx_hover_01: Sound,
    pub sfx_planet_place_01: Sound,
    pub sfx_planet_place_deny_01: Sound,
    pub sfx_planet_remove_01: Sound,
    pub sfx_planet_remove_deny_01: Sound,
    pub sfx_explosion_01: Sound,
    pub sfx_level_start_01: Sound,

    pub music_level_end_01: Sound,
}

impl GameState {
    pub async fn new() -> Self {
        let styles = Styles::new();

        GameState::show_loading_screen(&styles);

        let mouse_pos = f32::Vec2::ZERO;
        let is_mouse_in_grid = false;
        let tile_highlighted_prev = IVec2::splat(-1);
        let tile_highlighted = IVec2::ZERO;

        let levels = GameState::create_levels(&styles);
        let level_active = Some(0);
        // let level_active = Some(levels.len() - 1);
        let planet_current_index = 0;

        let score = 0;

        let sim_step = 0;
        let sim_step_computed = 0;

        let texture_explosion_01 = load_texture("assets/explosion_01.png").await.unwrap();
        let texture_background_01 = load_texture("assets/background.png").await.unwrap();

        let sfx_hover_01 = load_sound("assets/sfx/hover_02.ogg").await.unwrap();
        let sfx_planet_place_01 = load_sound("assets/sfx/planet_place_01.ogg").await.unwrap();
        let sfx_planet_place_deny_01 = load_sound("assets/sfx/planet_place_deny_01.ogg")
            .await
            .unwrap();
        let sfx_planet_remove_01 = load_sound("assets/sfx/planet_remove_01.ogg").await.unwrap();
        let sfx_planet_remove_deny_01 = load_sound("assets/sfx/planet_remove_deny_01.ogg")
            .await
            .unwrap();
        let sfx_explosion_01 = load_sound("assets/sfx/explosion_01.ogg").await.unwrap();
        let sfx_level_start_01 = load_sound("assets/sfx/level_start_01.ogg").await.unwrap();

        let music_level_end_01 = load_sound("assets/music/planet_001_short.ogg")
            .await
            .unwrap();

        Self {
            styles,

            mouse_pos,
            is_mouse_in_grid,
            tile_highlighted_prev,
            tile_highlighted,

            level_active,
            levels,
            planet_current_index,

            score,

            sim_step,
            sim_step_computed,

            texture_explosion_01,
            texture_background_01,

            sfx_hover_01,
            sfx_planet_place_01,
            sfx_planet_place_deny_01,
            sfx_planet_remove_01,
            sfx_planet_remove_deny_01,
            sfx_explosion_01,
            sfx_level_start_01,

            music_level_end_01,
        }
    }

    pub fn current_level_mut(&mut self) -> Option<&mut Level> {
        match self.level_active {
            None => return None,
            Some(i) => return Some(&mut self.levels[i]),
        }
    }

    pub fn current_level(&self) -> Option<&Level> {
        match self.level_active {
            None => return None,
            Some(i) => return Some(&self.levels[i]),
        }
    }

    fn show_loading_screen(styles: &Styles) {
        clear_background(styles.colors.black_1);
        let font_size = 16.0;
        let message_size = 148.0;
        let pos_message_x = SCREEN_W / 2.0 - message_size / 2.0;
        let pos_message_y = (SCREEN_H / 2.0) - font_size;
        draw_rectangle(
            pos_message_x - 2.0,
            pos_message_y - 2.0,
            message_size + 4.0,
            16.0 + 4.0,
            styles.colors.yellow_4,
        );
        draw_rectangle(
            pos_message_x,
            pos_message_y,
            message_size,
            16.0,
            styles.colors.yellow_1,
        );
        draw_scaled_text(
            "LOADING...",
            pos_message_x,
            pos_message_y + font_size / 1.333,
            font_size,
            &styles.colors.black_1,
        );
    }

    pub fn create_levels(styles: &Styles) -> Vec<Level> {
        use crate::planet::PlanetState::*;

        // TODO(Jesus): Make planets of different sizes.
        // TODO(Jesus): Consider adding more colors.

        let levels = vec![
            Level::new(
                "1. Planet",
                IVec2::new(3, 3),
                vec![Planet::new(0b0000, Pending, true, 8.0, styles.colors.white)],
            ),
            Level::new(
                "2. Gravity",
                IVec2::new(3, 3),
                vec![
                    Planet::new(0b0000, Pending, true, 9.0, styles.colors.yellow_1),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 0)),
                        false,
                        8.0,
                        styles.colors.white,
                    ),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 1)),
                        false,
                        8.0,
                        styles.colors.red_light,
                    ),
                ],
            ),
            Level::new(
                "3. Direction",
                IVec2::new(3, 3),
                vec![
                    Planet::new(0b0000, Pending, true, 9.0, styles.colors.red_light),
                    Planet::new(
                        0b0010,
                        Placed(IVec2::new(0, 0)),
                        false,
                        8.0,
                        styles.colors.white,
                    ),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 1)),
                        false,
                        8.0,
                        styles.colors.yellow_2,
                    ),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 2)),
                        false,
                        8.0,
                        styles.colors.red_light,
                    ),
                ],
            ),
            Level::new(
                "4. Two planets",
                IVec2::new(3, 3),
                vec![
                    Planet::new(0b0000, Pending, true, 9.0, styles.colors.red_light),
                    Planet::new(0b0000, Pending, true, 7.0, styles.colors.white),
                    Planet::new(
                        0b0011,
                        Placed(IVec2::new(0, 1)),
                        false,
                        8.0,
                        styles.colors.yellow_2,
                    ),
                    Planet::new(
                        0b0011,
                        Placed(IVec2::new(0, 2)),
                        false,
                        8.0,
                        styles.colors.red_light,
                    ),
                ],
            ),
            Level::new(
                "5. Three planets",
                IVec2::new(4, 4),
                vec![
                    Planet::new(0b0000, Pending, true, 9.0, styles.colors.red_light),
                    Planet::new(0b0001, Pending, true, 8.0, styles.colors.yellow_4),
                    Planet::new(0b0000, Pending, true, 7.0, styles.colors.red_light),
                    Planet::new(
                        0b0011,
                        Placed(IVec2::new(0, 0)),
                        false,
                        8.0,
                        styles.colors.yellow_2,
                    ),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 2)),
                        false,
                        8.0,
                        styles.colors.white,
                    ),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 3)),
                        false,
                        8.0,
                        styles.colors.white,
                    ),
                ],
            ),
            Level::new(
                "6. Test",
                IVec2::new(4, 4),
                vec![
                    Planet::new(0b0001, Pending, true, 8.0, styles.colors.yellow_4),
                    Planet::new(0b0011, Pending, true, 9.0, styles.colors.red_light),
                    Planet::new(0b0000, Pending, true, 7.0, styles.colors.white),
                    Planet::new(
                        0b0000,
                        Placed(IVec2::new(0, 0)),
                        false,
                        8.0,
                        styles.colors.yellow_2,
                    ),
                    Planet::new(
                        0b0000,
                        Placed(IVec2::new(0, 2)),
                        false,
                        8.0,
                        styles.colors.white,
                    ),
                    Planet::new(
                        0b0000,
                        Placed(IVec2::new(0, 3)),
                        false,
                        8.0,
                        styles.colors.white,
                    ),
                ],
            ),
            Level::new(
                "7. Obstacle",
                IVec2::new(10, 2),
                vec![
                    Planet::new(0b0000, Pending, true, 7.0, styles.colors.white),
                    Planet::new(0b0010, Pending, true, 9.0, styles.colors.red_light),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 0)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 1)),
                        false,
                        8.0,
                        styles.colors.white,
                    ),
                ],
            ),
            Level::new(
                "8. Swap",
                IVec2::new(6, 2),
                vec![
                    Planet::new(0b0000, Pending, true, 7.0, styles.colors.white),
                    Planet::new(0b0010, Pending, true, 9.0, styles.colors.red_light),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 0)),
                        false,
                        8.0,
                        styles.colors.yellow_2,
                    ),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 1)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                ],
            ),
            Level::new(
                "9. Squeeze",
                IVec2::new(5, 5),
                vec![
                    Planet::new(0b1111, Pending, true, 9.0, styles.colors.white),
                    Planet::new(
                        0b0000,
                        Placed(IVec2::new(2, 0)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b1000,
                        Placed(IVec2::new(1, 1)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b0000,
                        Placed(IVec2::new(4, 2)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b0000,
                        Placed(IVec2::new(0, 2)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b0010,
                        Placed(IVec2::new(3, 3)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b0000,
                        Placed(IVec2::new(2, 4)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                ],
            ),
            Level::new(
                "10. Intervention",
                IVec2::new(5, 5),
                vec![
                    Planet::new(0b0000, Pending, true, 9.0, styles.colors.white),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(2, 0)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b1000,
                        Placed(IVec2::new(1, 1)),
                        true,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b1000,
                        Placed(IVec2::new(4, 2)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b0100,
                        Placed(IVec2::new(0, 2)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b0010,
                        Placed(IVec2::new(3, 3)),
                        true,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b1000,
                        Placed(IVec2::new(2, 4)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                ],
            ),
            Level::new(
                "11. Defuse",
                IVec2::new(9, 9),
                vec![
                    Planet::new(0b0100, Pending, true, 9.0, styles.colors.white),
                    Planet::new(0b0010, Pending, true, 9.0, styles.colors.white),
                    Planet::new(
                        0b1111,
                        Placed(IVec2::new(0, 0)),
                        false,
                        8.0,
                        styles.colors.yellow_2,
                    ),
                    Planet::new(
                        0b1111,
                        Placed(IVec2::new(0, 8)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b1111,
                        Placed(IVec2::new(8, 0)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b1111,
                        Placed(IVec2::new(8, 8)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                ],
            ),
            Level::new(
                "12. Loop",
                IVec2::new(7, 4),
                vec![
                    Planet::new(0b0100, Pending, true, 9.0, styles.colors.white),
                    Planet::new(0b0010, Pending, true, 9.0, styles.colors.white),
                    Planet::new(0b1000, Pending, true, 9.0, styles.colors.white),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 0)),
                        false,
                        8.0,
                        styles.colors.yellow_2,
                    ),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 1)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 2)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                    Planet::new(
                        0b0001,
                        Placed(IVec2::new(0, 3)),
                        false,
                        8.0,
                        styles.colors.yellow_1,
                    ),
                ],
            ),
        ];

        levels
    }
}

#[derive(Clone)]
pub struct Level {
    pub name: &'static str,
    pub planets: Vec<Planet>,
    pub planets_original: Vec<Planet>,
    pub grid_tiles: IVec2,

    pub score: i32,

    pub was_failed: bool,
    pub was_stable: bool,

    pub is_failed: bool,
    pub is_stable: bool,

    pub is_setup: bool,
}

impl Level {
    pub fn new(name: &'static str, grid_tiles: IVec2, planets: Vec<Planet>) -> Self {
        let was_failed = false;
        let was_stable = false;

        let is_failed = false;
        let is_stable = false;

        let is_setup = false;

        let score = 0;

        let planets_original = planets.clone();

        Self {
            name,
            planets,
            planets_original,
            grid_tiles,

            score,

            was_failed,
            was_stable,

            is_failed,
            is_stable,

            is_setup,
        }
    }

    pub fn grid_size_px(&self) -> f32::Vec2 {
        f32::Vec2::new(
            TILE_SIZE_X * self.grid_tiles.x as f32,
            TILE_SIZE_Y * self.grid_tiles.y as f32,
        )
    }

    pub fn grid_offset(&self) -> f32::Vec2 {
        let grid_size_px = self.grid_size_px();

        f32::Vec2::new(
            (SCREEN_W - grid_size_px.x) / 2.0,
            (SCREEN_H - grid_size_px.y) / 2.0,
        )
    }

    pub fn reset(&mut self) {
        self.planets = self.planets_original.clone();

        self.was_failed = false;
        self.was_stable = false;

        self.is_failed = false;
        self.is_stable = false;

        self.is_setup = false;

        self.score = 0;
    }
}
