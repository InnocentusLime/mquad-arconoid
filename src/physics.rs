use macroquad::prelude::*;

const BALL_SPEED: f32 = 120.0;
const BOX_PER_LINE: usize = 30;
const BOX_LINE_COUNT: usize = 13;
const BOX_WIDTH: f32 = 20.0;
const BOX_HEIGHT: f32 = 10.0;
const BALL_RADIUS: f32 = 6.0;
const MAX_X: f32 = BOX_WIDTH * (BOX_PER_LINE as f32);
const MAX_Y: f32 = 410.0;
const PLAYER_WIDTH: f32 = 80.0;
const PLAYER_HEIGHT: f32 = 10.0;

pub struct Physics {
    player_x: f32,
    ball_pos: Vec2,
    ball_dir: Vec2,
    boxes: [bool; BOX_PER_LINE * BOX_LINE_COUNT],
}

impl Physics {
    pub fn new() -> Self {
        Self {
            player_x: 0.0,
            ball_pos: vec2(30.0, 200.0),
            ball_dir: -vec2(1.0, -1.0),
            boxes: [true; BOX_PER_LINE * BOX_LINE_COUNT],
        }
    }

    pub fn update(&mut self, dt: f32) {
        let offset = self.ball_dir * BALL_SPEED * dt;
        let mut new_ball_pos = self.ball_pos + offset;

        if new_ball_pos.x - BALL_RADIUS < 0.0 {
            self.ball_dir.x *= -1.0;
            new_ball_pos.x = BALL_RADIUS;
        }

        if new_ball_pos.x + BALL_RADIUS > MAX_X {
            self.ball_dir.x *= -1.0;
            new_ball_pos.x = MAX_X - BALL_RADIUS;
        }

        if new_ball_pos.y - BALL_RADIUS < 0.0 {
            self.ball_dir.y *= -1.0;
            new_ball_pos.y = BALL_RADIUS;
        }

        if new_ball_pos.y + BALL_RADIUS > MAX_Y {
            self.ball_dir.y *= -1.0;
            new_ball_pos.y = MAX_Y - BALL_RADIUS;
        }

        self.ball_pos = new_ball_pos;
    }

    pub fn draw(&self) {
        for by in 0..BOX_LINE_COUNT {
            for bx in 0..BOX_PER_LINE {
                if !self.boxes[Self::box_id(bx, by)] {
                    continue;
                }

                let box_rect = Self::box_rect(bx, by);

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

        draw_rectangle(
            self.player_x,
            MAX_Y - PLAYER_HEIGHT,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            YELLOW
        );

        draw_circle(self.ball_pos.x, self.ball_pos.y, BALL_RADIUS, YELLOW);
    }

    fn box_rect(x: usize, y: usize) -> Rect {
        Rect {
            x: (x as f32) * BOX_WIDTH,
            y: (y as f32) * BOX_HEIGHT,
            w: BOX_WIDTH,
            h: BOX_HEIGHT,
        }
    }

    fn box_id(x: usize, y: usize) -> usize {
        x + y * BOX_PER_LINE
    }
}