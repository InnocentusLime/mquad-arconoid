use macroquad::prelude::*;
use physics::Physics;

mod physics;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut phys = Physics::new();

    loop {
        clear_background(RED);

        let dt = get_frame_time();

        if is_key_down(KeyCode::A) {
            phys.move_player(dt, false);
        }
        if is_key_down(KeyCode::D) {
            phys.move_player(dt, true);
        }

        phys.update(get_frame_time());
        phys.draw();

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}