use macroquad::prelude::*;

use crate::physics::{self, Physics, BALL_RADIUS, BOX_LINE_COUNT, MAX_X, MAX_Y};



pub struct Render {
    ball1: Texture2D,
    ball2: Texture2D,
    ball3: Texture2D,
    pla1: Texture2D,
    pla2: Texture2D,
    pla3: Texture2D,
    bricks: Texture2D,
    outline: Texture2D,
}

impl Render {
    pub async fn new() -> Self {
        let outline = load_texture("assets/brick_outline.png").await.unwrap();
        let bricks = load_texture("assets/bricks.png").await.unwrap();
        bricks.set_filter(FilterMode::Nearest);
        outline.set_filter(FilterMode::Nearest);

        Self {
            /* */
            ball1: load_texture("assets/ball1.png").await.unwrap(),
            ball2: load_texture("assets/ball2.png").await.unwrap(),
            ball3: load_texture("assets/ball3.png").await.unwrap(),
            /* */
            pla1: load_texture("assets/pl1.png").await.unwrap(),
            pla2: load_texture("assets/pl2.png").await.unwrap(),
            pla3: load_texture("assets/pl3.png").await.unwrap(),
            /* */
            bricks,
            outline,
        }
    }

    pub fn draw(&mut self, phys: &Physics, t: f32) {
        clear_background(Color {
            r: 0.0,
            g: 0.0,
            b: 0.12,
            a: 1.0,
        });

        let view_width = (screen_width() / screen_height()) * physics::MAX_Y;
        let mut cam = Camera2D::from_display_rect(Rect {
            x: -(view_width - physics::MAX_X) / 2.0,
            y: 0.0,
            w: view_width,
            h: physics::MAX_Y,
        });
        cam.zoom.y *= -1.0;

        set_camera(&cam);

        for by in 0..physics::BOX_LINE_COUNT {
            for bx in 0..physics::BOX_PER_LINE {
                if !phys.boxes[Physics::box_id(bx, by)] {
                    continue;
                }

                let box_rect = Physics::box_rect(bx, by);
                let idx = ((53 + bx) * 53 + by) % 16;
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
        // draw_rectangle(
        //     rect.x,
        //     rect.y,
        //     rect.w,
        //     rect.h,
        //     YELLOW
        // );


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
        // draw_circle(
        //     phys.ball_pos.x,
        //     phys.ball_pos.y,
        //     physics::BALL_RADIUS,
        //     YELLOW
        // );
    }
}