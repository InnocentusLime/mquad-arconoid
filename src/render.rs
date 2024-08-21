use macroquad::prelude::*;

use crate::physics::{self, Physics, MAX_X, MAX_Y};



pub struct Render {

}

impl Render {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, phys: &Physics) {
        clear_background(BLACK);

        draw_rectangle(
            0.0, 0.0,
            MAX_X, MAX_Y,
            DARKBLUE
        );

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

                draw_rectangle(
                    box_rect.x,
                    box_rect.y,
                    box_rect.w,
                    box_rect.h,
                    BLUE
                );
                draw_rectangle_lines(
                    box_rect.x,
                    box_rect.y,
                    box_rect.w,
                    box_rect.h,
                    1.0,
                    BLACK,
                );
            }
        }

        let rect = phys.player_rect();

        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            YELLOW
        );

        draw_circle(
            phys.ball_pos.x,
            phys.ball_pos.y,
            physics::BALL_RADIUS,
            YELLOW
        );
    }
}