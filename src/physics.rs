use macroquad::prelude::*;

const PUSH_EPSILON: f32 = 0.001;
const PLAYER_SPEED: f32 = 160.0;
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
        let mut boxes = [true; BOX_PER_LINE * BOX_LINE_COUNT];

        for y in 0..BOX_LINE_COUNT {
            boxes[Self::box_id(4, y)] = false;
            boxes[Self::box_id(5, y)] = false;
            boxes[Self::box_id(6, y)] = false;
            boxes[Self::box_id(7, y)] = false;
            boxes[Self::box_id(8, y)] = false;
            boxes[Self::box_id(9, y)] = false;
            boxes[Self::box_id(10, y)] = false;
            boxes[Self::box_id(11, y)] = false;
            boxes[Self::box_id(12, y)] = false;
            boxes[Self::box_id(13, y)] = false;
            boxes[Self::box_id(14, y)] = false;
        }

        Self {
            player_x: 0.0,
            ball_pos: vec2(30.0, 180.0),
            ball_dir: vec2(-1.0, -1.0).normalize(),
            boxes,
        }
    }

    pub fn move_player(&mut self, dt: f32, right: bool) {
        let mut dx = PLAYER_SPEED * dt;
        if !right { dx *= -1.0; }

        self.player_x += dx;
    }

    pub fn update(&mut self, dt: f32) {
        let offset = self.ball_dir * BALL_SPEED * dt;
        let mut new_ball_pos = self.ball_pos + offset;

        self.player_x = self.player_x.clamp(0.0, MAX_X - PLAYER_WIDTH);

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

        for by in 0..BOX_LINE_COUNT {
            for bx in 0..BOX_PER_LINE {
                let box_rect = Self::box_rect(bx, by);

                if !self.boxes[Self::box_id(bx, by)] {
                    continue;
                }

                if !Self::ball_in_rect(new_ball_pos, box_rect) {
                    continue;
                }

                self.boxes[Self::box_id(bx, by)] = false;

                if self.ball_pos.x + BALL_RADIUS >= box_rect.left() &&
                    self.ball_pos.x - BALL_RADIUS <= box_rect.right()
                {
                    self.ball_dir.y *= -1.0;
                    if self.ball_pos.y > box_rect.bottom() {
                        new_ball_pos.y = box_rect.bottom() + BALL_RADIUS + PUSH_EPSILON;
                    } else {
                        new_ball_pos.y = box_rect.top() - BALL_RADIUS - PUSH_EPSILON;
                    }
                } else {
                    self.ball_dir.x *= -1.0;
                    if self.ball_pos.x > box_rect.right() {
                        new_ball_pos.x = box_rect.right() + BALL_RADIUS + PUSH_EPSILON;
                    } else {
                        new_ball_pos.x = box_rect.left() - BALL_RADIUS - PUSH_EPSILON;
                    }
                }
            }
        }

        let player_rect = self.player_rect();
        if Self::ball_in_rect(self.ball_pos, player_rect) {
            self.ball_dir.y *= -1.0;
            new_ball_pos.y = player_rect.y - BALL_RADIUS - PUSH_EPSILON;
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

        let rect = self.player_rect();

        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            YELLOW
        );

        draw_circle(self.ball_pos.x, self.ball_pos.y, BALL_RADIUS, YELLOW);
    }

    fn player_rect(&self) -> Rect {
        Rect {
            x: self.player_x,
            y: MAX_Y - PLAYER_HEIGHT,
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
        }
    }

    fn ball_in_rect(pos: Vec2, rect: Rect) -> bool {
        pos.x + BALL_RADIUS >= rect.left() &&
        pos.x - BALL_RADIUS <= rect.right() &&
        pos.y + BALL_RADIUS >= rect.top() &&
        pos.y - BALL_RADIUS <= rect.bottom()
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