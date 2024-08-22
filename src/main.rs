use macroquad::prelude::*;
use physics::Physics;
use render::Render;

mod physics;
mod render;

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
    let mut render = Render::new().await;
    let mut broken = Vec::with_capacity(3);

    let mut t = 0.0;
    loop {
        let dt = get_frame_time();
        t += dt;

        if is_key_down(KeyCode::A) {
            phys.move_player(dt, false);
        }
        if is_key_down(KeyCode::D) {
            phys.move_player(dt, true);
        }

        broken.clear();

        let old_blocks = phys.boxes;
        let hit_floor = phys.update(get_frame_time());
        for by in 0..physics::BOX_LINE_COUNT {
            for bx in 0..physics::BOX_PER_LINE {
                if old_blocks[by][bx] == phys.boxes[by][bx] {
                    continue;
                }
                broken.push((bx, by));
            }
        }

        if hit_floor {
            // break;
        }

        render.draw(&phys, t, &broken);

        next_frame().await
    }
}