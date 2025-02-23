use macroquad::{
    color::{self, Color},
    math::{f32, IVec2},
    shapes::{draw_circle, draw_line, draw_poly, draw_rectangle},
    texture::{draw_texture, draw_texture_ex, DrawTextureParams},
};

use crate::{constants::*, game_state::GameState, text::draw_scaled_text};

#[derive(Clone, Copy)]
pub struct Planet {
    pub state: PlanetState,
    pub size: f32,
    pub color: Color,

    pub render_pos: f32::Vec2,

    /// Up, down, left, right
    pub gravity_field: u8,

    pub is_removable: bool,

    pub sim_tile_delta: IVec2,
}

impl Planet {
    pub fn new(
        gravity_field: u8,
        state: PlanetState,
        is_removable: bool,
        size: f32,
        color: Color,
    ) -> Self {
        let sim_tile_next = IVec2::ZERO;

        let render_pos = f32::Vec2::NEG_ONE;

        Self {
            gravity_field,
            state,
            is_removable,

            render_pos,

            size,
            color,

            sim_tile_delta: sim_tile_next,
        }
    }

    pub fn place(&mut self, tile: IVec2, grid_offset: f32::Vec2) {
        self.state = PlanetState::Placed(tile);

        if self.render_pos.x < 0.0 || self.render_pos.y < 0.0 {
            self.render_pos.x = tile.x as f32 * TILE_SIZE_X + grid_offset.x + TILE_SIZE_X / 2.0;
            self.render_pos.y = tile.y as f32 * TILE_SIZE_Y + grid_offset.y + TILE_SIZE_Y / 2.0;
        }
    }

    pub fn remove(&mut self) {
        self.state = PlanetState::Pending;
        self.render_pos.x = -1.0;
        self.render_pos.y = -1.0;
    }

    pub fn has_gravity_up(&self) -> bool {
        self.gravity_field & 0b1000 > 0
    }
    pub fn has_gravity_down(&self) -> bool {
        self.gravity_field & 0b0100 > 0
    }
    pub fn has_gravity_left(&self) -> bool {
        self.gravity_field & 0b0010 > 0
    }
    pub fn has_gravity_right(&self) -> bool {
        self.gravity_field & 0b0001 > 0
    }

    pub fn render(&mut self, game_state: &GameState) {
        match self.state {
            PlanetState::Pending => {
                draw_scaled_text(
                    "Place planet",
                    8.0,
                    16.0,
                    16.0,
                    &game_state.styles.colors.white,
                );

                let x = game_state.mouse_pos.x;
                let y = game_state.mouse_pos.y;

                draw_circle(48.0, 48.0, self.size * 2.0, self.color);
                self.draw_gravity_arrows(48.0, 48.0, 2.0, game_state);
                draw_circle(x, y, self.size, self.color);
                self.draw_gravity_arrows(x, y, 1.0, game_state);
            }
            PlanetState::Placed(tile) => {
                let grid_offset: f32::Vec2;

                match game_state.current_level() {
                    Some(level) => {
                        grid_offset = level.grid_offset();
                    }
                    None => {
                        grid_offset = f32::Vec2::ZERO;
                    }
                }

                let target = f32::Vec2::new(
                    tile.x as f32 * TILE_SIZE_X + grid_offset.x + TILE_SIZE_X / 2.0,
                    tile.y as f32 * TILE_SIZE_Y + grid_offset.y + TILE_SIZE_Y / 2.0,
                );

                // Initialize render_pos if needed
                if self.render_pos.x < 0.0 || self.render_pos.y < 0.0 {
                    self.render_pos.x = target.x;
                    self.render_pos.y = target.y;
                }

                self.render_pos = self.render_pos.move_towards(target, 2.0);

                if !self.is_removable {
                    let mut color = game_state.styles.colors.red_dark;
                    color.a = 0.8;
                    let cell_w = TILE_SIZE_X;
                    let cell_h = TILE_SIZE_Y;
                    draw_rectangle(
                        self.render_pos.x - cell_w / 2.0,
                        self.render_pos.y - cell_h / 2.0,
                        cell_w,
                        cell_h,
                        color,
                    );
                }
                draw_circle(self.render_pos.x, self.render_pos.y, self.size, self.color);
                self.draw_gravity_arrows(self.render_pos.x, self.render_pos.y, 1.0, game_state);
            }
            PlanetState::Colliding(tile) => {
                let grid_offset: f32::Vec2;

                match game_state.current_level() {
                    Some(level) => {
                        grid_offset = level.grid_offset();
                    }
                    None => {
                        grid_offset = f32::Vec2::ZERO;
                    }
                }

                let x = tile.x as f32 * TILE_SIZE_X + grid_offset.x;
                let y = tile.y as f32 * TILE_SIZE_Y + grid_offset.y;

                let texture = &game_state.texture_explosion_01;
                draw_texture(texture, x, y, color::WHITE);
            }
        }
    }

    pub fn render_stack(&self, index: usize, game_state: &GameState) {
        let x = SCREEN_W - 32.0;
        let y = 32.0 + 36.0 * index as f32;
        let scale = 2.0;

        match self.state {
            PlanetState::Pending => {
                if index == game_state.planet_current_index {
                    let mut color = game_state.styles.colors.yellow_1;
                    color.a = 0.2;
                    draw_rectangle(
                        x - self.size * scale * 1.2,
                        y - self.size * scale * 1.2,
                        self.size * 2.0 * scale * 1.2,
                        self.size * 2.0 * scale * 1.2,
                        color,
                    );
                }
                draw_circle(x, y, self.size * scale, self.color);
                self.draw_gravity_arrows(x, y, scale, game_state);
            }
            PlanetState::Placed(_) => {
                draw_circle(x, y, self.size * scale, self.color);
                self.draw_gravity_arrows(x, y, scale, game_state);
                let mut color_line = game_state.styles.colors.red_light;
                color_line.a = 0.8;
                draw_line(
                    x - self.size * scale,
                    y - self.size * scale,
                    x + self.size * scale,
                    y + self.size * scale,
                    4.0,
                    color_line,
                );
                draw_line(
                    x + self.size * scale,
                    y - self.size * scale,
                    x - self.size * scale,
                    y + self.size * scale,
                    4.0,
                    color_line,
                );
            }
            PlanetState::Colliding(_) => {
                draw_circle(x, y, self.size * scale, self.color);
                self.draw_gravity_arrows(x, y, scale, game_state);
                let mut color_line = game_state.styles.colors.red_light;
                color_line.a = 0.8;
                draw_line(
                    x - self.size * scale,
                    y - self.size * scale,
                    x + self.size * scale,
                    y + self.size * scale,
                    4.0,
                    color_line,
                );
                draw_line(
                    x + self.size * scale,
                    y - self.size * scale,
                    x - self.size * scale,
                    y + self.size * scale,
                    4.0,
                    color_line,
                );

                let mut color = color::WHITE;
                color.a = 0.8;

                let texture = &game_state.texture_explosion_01;
                let texture_size = texture.size();
                draw_texture_ex(
                    texture,
                    x - texture_size.x / 2.0,
                    y - texture_size.y / 2.0,
                    color,
                    DrawTextureParams {
                        dest_size: Some(f32::Vec2::new(texture_size.x, texture_size.y)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    fn draw_gravity_arrows(&self, x: f32, y: f32, scale: f32, game_state: &GameState) {
        let arrow_size = 4.0 * scale;
        let arrow_color = game_state.styles.colors.red_dark;
        if self.has_gravity_up() {
            draw_poly(x, y - arrow_size, 3, arrow_size, 90.0, arrow_color);
        }
        if self.has_gravity_down() {
            draw_poly(x, y + arrow_size, 3, arrow_size, -90.0, arrow_color);
        }
        if self.has_gravity_left() {
            draw_poly(x - arrow_size, y, 3, arrow_size, 0.0, arrow_color);
        }
        if self.has_gravity_right() {
            draw_poly(x + arrow_size, y, 3, arrow_size, -180.0, arrow_color);
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum PlanetState {
    Pending,
    Placed(IVec2),
    Colliding(IVec2),
}
