use macroquad::prelude::*;
use physics::Physics;

mod physics;

fn window_conf() -> Conf {
    Conf {
        window_title: "Funny Arcanoid".to_owned(),
        high_dpi: true,
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
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

        let hit_floor = phys.update(get_frame_time());
        if hit_floor {
            break;
        }

        phys.draw();

        next_frame().await
    }
}