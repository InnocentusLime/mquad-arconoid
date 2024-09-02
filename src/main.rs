use macroquad::{audio::{self, load_sound, PlaySoundParams}, prelude::*};
use miniquad::window::set_window_size;
use physics::Physics;
use render::Render;
use sys::*;
use ui::Ui;

mod physics;
mod render;
mod sys;
mod ui;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GameState {
    Start,
    Active,
    GameOver,
    Win,
    Paused,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Boring Arcanoid".to_owned(),
        high_dpi: true,
        window_width: 1600,
        window_height: 900,
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
    let ui = Ui::new().await;

    let dead = load_sound("assets/dead.wav").await.unwrap();
    let bsound = load_sound("assets/break.wav").await.unwrap();
    let bounce = load_sound("assets/ball.wav").await.unwrap();
    let mut fullscreen = window_conf().fullscreen;

    // Save old size as leaving fullscreen will give window a different size
    // This value is our best bet as macroquad doesn't allow us to get window size
    let old_size = (window_conf().window_width, window_conf().window_height);

    done_loading();

    loop {
        let mut broken = None;
        let dt = get_frame_time();

        clear_background(Color {
            r: 0.0,
            g: 0.0,
            b: 0.12,
            a: 1.0,
        });

        let ui_model = ui.update(state);
        phys.new_frame();
        let prev_state = state;

        if ui_model.fullscreen_toggle_requested() {
            // NOTE: macroquad does not update window config when it goes fullscreen
            set_fullscreen(!fullscreen);

            if fullscreen {
                set_window_size(old_size.0 as u32, old_size.1 as u32);
            }

            fullscreen = !fullscreen;
        }

        match state {
            GameState::Start => {
                if ui_model.confirmation_detected() {
                    state = GameState::Active;
                }
            },
            GameState::Active => {
                if ui_model.move_left() {
                    phys.move_player(dt, false);
                }
                if ui_model.move_right() {
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
                        audio::play_sound(
                            &bsound,
                            PlaySoundParams {
                                looped: false,
                                volume: 0.4,
                            }
                        );
                    }
                }

                if old_dir != phys.ball_dir && !block_break_played {
                    audio::play_sound(
                        &bounce,
                        PlaySoundParams {
                            looped: false,
                            volume: 0.23,
                        }
                    );
                }

                if hit_floor {
                    state = GameState::GameOver;
                    audio::play_sound(
                        &dead,
                        PlaySoundParams {
                            looped: false,
                            volume: 0.4,
                        }
                    );
                }

                if phys.boxes.iter().flat_map(|x| x.iter()).all(|x| !*x) {
                    state = GameState::Win;
                }

                if ui_model.pause_requested() {
                    state = GameState::Paused;
                }
            },
            GameState::GameOver => {
                if ui_model.confirmation_detected() {
                    phys = Physics::new();
                    state = GameState::Active;
                }
            },
            GameState::Win => {
                if ui_model.confirmation_detected() {
                    phys = Physics::new();
                    state = GameState::Active;
                }
            },
            GameState::Paused => {
                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::Active;
                }
            },
        };

        render.draw(state, &phys, prev_state, broken.into_iter());
        ui.draw(ui_model);

        next_frame().await
    }
}