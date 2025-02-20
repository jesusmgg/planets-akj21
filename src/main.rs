use macroquad::prelude::*;

#[macroquad::main("akj-21")]
async fn main() {
    loop {
        if is_key_pressed(macroquad::input::KeyCode::Q) {
            miniquad::window::quit();
        }

        clear_background(RED);

        draw_text("HELLO, WORLD!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
