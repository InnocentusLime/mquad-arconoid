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

        next_frame().await
    }
}