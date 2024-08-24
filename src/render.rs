use macroquad::prelude::*;

use crate::{physics::{self, Physics, BALL_RADIUS, BOX_HEIGHT, BOX_LINE_COUNT, BOX_WIDTH, PLAYER_HEIGHT, PLAYER_WIDTH}, GameState};
use macroquad_particles::{self as particles, BlendMode, ColorCurve, EmitterConfig};

fn trail() -> particles::EmitterConfig {
    particles::EmitterConfig {
        emitting: true,
        lifetime: 1.2,
        lifetime_randomness: 0.7,
        explosiveness: 0.01,
        amount: 15,
        initial_direction_spread: 0.4 * std::f32::consts::PI,
        initial_velocity: 100.0,
        size: 1.0,
        gravity: vec2(0.0, 1000.0),
        blend_mode: BlendMode::Alpha,
        emission_shape: macroquad_particles::EmissionShape::Sphere { radius: BALL_RADIUS },
        colors_curve: ColorCurve {
            start: Color::from_hex(0xDDFBFF),
            mid: BLANK,
            end: BLANK,
        },
        ..Default::default()
    }
}

fn explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        one_shot: true,
        emitting: false,
        lifetime: 0.3,
        lifetime_randomness: 0.7,
        explosiveness: 0.99,
        amount: 30,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 200.0,
        size: 1.5,
        gravity: vec2(0.0, 1000.0),
        blend_mode: BlendMode::Alpha,
        emission_shape: macroquad_particles::EmissionShape::Rect {
            width: BOX_WIDTH,
            height: BOX_HEIGHT,
        },
        colors_curve: ColorCurve {
            start: Color::from_hex(0x333354),
            mid: Color::from_hex(0x333354),
            end: BLACK,
        },
        ..Default::default()
    }
}

fn ball_explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        one_shot: true,
        emitting: false,
        lifetime: 1.0,
        lifetime_randomness: 0.7,
        explosiveness: 0.99,
        amount: 10,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 100.0,
        size: 20.0,
        gravity: vec2(0.0, -1000.0),
        blend_mode: BlendMode::Alpha,
        emission_shape: macroquad_particles::EmissionShape::Sphere { radius: BALL_RADIUS * 4.0 },
        initial_angular_velocity: 5.0,
        angular_accel: 0.0,
        angular_damping: 0.01,
        colors_curve: ColorCurve {
            start: Color::from_hex(0xDDFBFF),
            mid: Color { r: 1.0, g: 0.0, b: 0.0, a: 0.0, },
            end: BLANK,
        },
        ..Default::default()
    }
}

pub struct Render {
    ball1: Texture2D,
    ball2: Texture2D,
    ball3: Texture2D,
    pla1: Texture2D,
    pla2: Texture2D,
    pla3: Texture2D,
    bricks: Texture2D,
    outline: Texture2D,
    ball_emit: particles::Emitter,
    pl_emit: particles::Emitter,
    brick_emit: particles::Emitter,
    ball_exp: particles::Emitter,
    last_brick_break: Vec2,
}

impl Render {
    pub async fn new() -> Self {
        let sad = load_texture("assets/ded.png").await.unwrap();

        let this = Self {
            /* */
            ball1: load_texture("assets/ball1.png").await.unwrap(),
            ball2: load_texture("assets/ball2.png").await.unwrap(),
            ball3: load_texture("assets/ball3.png").await.unwrap(),
            /* */
            pla1: load_texture("assets/pl1.png").await.unwrap(),
            pla2: load_texture("assets/pl2.png").await.unwrap(),
            pla3: load_texture("assets/pl3.png").await.unwrap(),
            /* */
            bricks: load_texture("assets/bricks.png").await.unwrap(),
            outline: load_texture("assets/brick_outline.png").await.unwrap(),
            ball_emit: particles::Emitter::new(EmitterConfig {
                texture: None,
                ..trail()
            }),
            pl_emit: particles::Emitter::new(EmitterConfig {
                texture: None,
                ..trail()
            }),
            brick_emit:  particles::Emitter::new(EmitterConfig {
                texture: None,
                ..explosion()
            }),
            ball_exp:  particles::Emitter::new(EmitterConfig {
                texture: Some(sad),
                ..ball_explosion()
            }),
            last_brick_break: Vec2::ZERO,
        };

        this.ball1.set_filter(FilterMode::Nearest);
        this.ball2.set_filter(FilterMode::Nearest);
        this.ball3.set_filter(FilterMode::Nearest);
        this.pla1.set_filter(FilterMode::Nearest);
        this.pla2.set_filter(FilterMode::Nearest);
        this.pla3.set_filter(FilterMode::Nearest);
        this.bricks.set_filter(FilterMode::Nearest);
        this.outline.set_filter(FilterMode::Nearest);

        this
    }

    pub fn draw(
        &mut self,
        state: GameState,
        phys: &Physics,
        prev_state: GameState,
        mut broken: impl Iterator<Item = (usize, usize)>,
    ) {
        self.setup_cam();
        self.draw_blocks(phys);
        self.draw_player(phys);

        if matches!(state, GameState::Start) {
            draw_rectangle(
                -screen_width(), -screen_height(),
                2.0*screen_width(), 2.0*screen_height(),
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.12,
                    a: 0.5,
                }
            );
            draw_text(
                "Press SPACE to start the game",
                physics::MAX_X / 2.0 - 200.0,
                200.0,
                32.0,
                Color::from_hex(0xDDFBFF)
            );
        }

        if matches!(state, GameState::Active) {
            self.draw_ball(phys);
        }

        if let Some((bx, by)) = broken.next() {
            self.brick_emit.config.emitting = true;
            self.last_brick_break = vec2(
                BOX_WIDTH * (bx as f32 + 0.5),
                BOX_HEIGHT * (by as f32 + 0.6),
            );
        }
        self.brick_emit.draw(self.last_brick_break);

        if prev_state == GameState::Active && state == GameState::GameOver {
            self.ball_exp.config.emitting = true;
        }
        self.ball_exp.draw(phys.ball_pos);

        if matches!(state, GameState::GameOver) {
            draw_rectangle(
                -screen_width(), -screen_height(),
                2.0*screen_width(), 2.0*screen_height(),
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.12,
                    a: 0.5,
                }
            );
            draw_text(
                "Game Over. Press Space to restart",
                physics::MAX_X / 2.0 - 220.0,
                200.0,
                32.0,
                Color::from_hex(0xDDFBFF)
            );
        }

    }

    fn setup_cam(&mut self) {
        let view_width = (screen_width() / screen_height()) * physics::MAX_Y;
        let mut cam = Camera2D::from_display_rect(Rect {
            x: -(view_width - physics::MAX_X) / 2.0,
            y: 0.0,
            w: view_width,
            h: physics::MAX_Y,
        });
        cam.zoom.y *= -1.0;

        set_camera(&cam);
    }

    fn draw_ball(&mut self, phys: &Physics) {
        let t = get_time() as f32;
        let tex = [&self.ball1, &self.ball2, &self.ball3];
        let tex = tex[(t * 5.0) as usize % 3];
        draw_texture_ex(
            tex,
            phys.ball_pos.x - BALL_RADIUS,
            phys.ball_pos.y - BALL_RADIUS,
            WHITE,
            DrawTextureParams {
                dest_size: Some(2.0 * vec2(
                    physics::BALL_RADIUS,
                    physics::BALL_RADIUS
                )),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
        self.ball_emit.config.initial_direction = -phys.ball_dir;
        self.ball_emit.config.gravity = phys.ball_dir;
        self.ball_emit.draw(phys.ball_pos);
    }

    fn draw_player(&mut self, phys: &Physics) {
        let t = get_time() as f32;
        let rect = phys.player_rect();

        let tex = [&self.pla1, &self.pla2, &self.pla3];
        let tex = tex[(t * 5.0) as usize % 3];
        draw_texture_ex(
            tex,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    rect.w,
                    rect.h * 1.3,
                )),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
        if phys.player_delta == 0.0 {
            self.pl_emit.config.emitting = false;
        } else {
            self.pl_emit.config.emitting = true;
            self.pl_emit.config.initial_direction = -vec2(phys.player_delta, 0.0).normalize();
            self.pl_emit.config.gravity = vec2(phys.player_delta, 0.0).normalize();
        }

        self.pl_emit.draw(vec2(
            rect.x + PLAYER_WIDTH / 2.0,
            rect.y + PLAYER_HEIGHT
        ) - vec2(phys.player_delta, 0.0).normalize_or_zero() * PLAYER_WIDTH / 2.0);
    }

    fn draw_blocks(&mut self, phys: &Physics) {
        for by in 0..physics::BOX_LINE_COUNT {
            for bx in 0..physics::BOX_PER_LINE {
                if !phys.boxes[by][bx] {
                    continue;
                }

                let box_rect = Physics::box_rect(bx, by);
                let mut idx = ((53 + bx) * 53 + by) % 16;
                idx = (idx + (get_time() / 1.0) as usize) % 16;
                let tx = idx % 4;
                let ty = idx / 4;

                let brick_col = Color {
                    r: (by as f32) / (BOX_LINE_COUNT as f32) * 0.5 + 0.5,
                    g: (by as f32) / (BOX_LINE_COUNT as f32) * 0.5 + 0.5,
                    b: (by as f32) / (BOX_LINE_COUNT as f32) * 0.5 + 0.5,
                    a: 1.0,
                };

                draw_texture_ex(&self.outline,
                    box_rect.x - 2.0,
                    box_rect.y - 2.0,
                    brick_col,
                    DrawTextureParams {
                        dest_size: Some(vec2(box_rect.w + 4.0, box_rect.h + 4.0)),
                        source: None,
                        rotation: 0.0,
                        flip_x: (idx % 4) == 0,
                        flip_y: (idx % 3) == 0,
                        pivot: None,
                    },
                );
                draw_texture_ex(&self.bricks,
                    box_rect.x,
                    box_rect.y,
                    brick_col,
                    DrawTextureParams {
                        dest_size: Some(vec2(box_rect.w, box_rect.h)),
                        source: Some(Rect {
                            x: (tx as f32) * 32.0,
                            y: (ty as f32) * 16.0,
                            w: 32.0,
                            h: 16.0,
                        }),
                        rotation: 0.0,
                        flip_x: (idx % 4) == 0,
                        flip_y: (idx % 3) == 0,
                        pivot: None,
                    },
                );
            }
        }
    }
}