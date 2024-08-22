use macroquad::{audio::{self, load_sound}, prelude::*};
use physics::Physics;
use render::Render;

mod physics;
mod render;

#[derive(Clone, Copy, Debug)]
enum GameState {
    Start,
    Active,
    GameOver,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Boring Arcanoid".to_owned(),
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
    let mut state = GameState::Start;

    let bsound = load_sound("assets/break.wav").await.unwrap();
    let bounce = load_sound("assets/ball.wav").await.unwrap();

    loop {
        let mut broken = None;
        let dt = get_frame_time();

        match state {
            GameState::Start => {
                if is_key_pressed(KeyCode::Space) {
                    state = GameState::Active;
                }
            },
            GameState::Active => {
                if is_key_down(KeyCode::A) {
                    phys.move_player(dt, false);
                }
                if is_key_down(KeyCode::D) {
                    phys.move_player(dt, true);
                }

                let old_dir = phys.ball_dir;
                let old_blocks = phys.boxes;
                let hit_floor = phys.update(get_frame_time());
                let mut block_break_played = false;

                for by in 0..physics::BOX_LINE_COUNT {
                    for bx in 0..physics::BOX_PER_LINE {
                        if old_blocks[by][bx] == phys.boxes[by][bx] {
                            continue;
                        }
                        broken = Some((bx, by));
                        block_break_played = true;
                        audio::play_sound_once(&bsound);
                    }
                }

                if old_dir != phys.ball_dir && !block_break_played {
                    audio::play_sound_once(&bounce);
                }

                if hit_floor {
                    state = GameState::GameOver
                }
            },
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    phys = Physics::new();
                    state = GameState::Active;
                }

            },
        };

        render.draw(state, &phys, broken.into_iter());

        next_frame().await
    }
}