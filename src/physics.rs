use macroquad::prelude::*;

pub const PUSH_EPSILON: f32 = 0.001;
pub const PLAYER_SPEED: f32 = 170.0;
pub const BALL_SPEED: f32 = 180.0;
pub const BOX_PER_LINE: usize = 15;
pub const BOX_LINE_COUNT: usize = 8;
pub const BOX_WIDTH: f32 = 40.0;
pub const BOX_HEIGHT: f32 = 20.0;
pub const BALL_RADIUS: f32 = 6.0;
pub const MAX_X: f32 = BOX_WIDTH * (BOX_PER_LINE as f32);
pub const MAX_Y: f32 = 410.0;
pub const PLAYER_WIDTH: f32 = 80.0;
pub const PLAYER_HEIGHT: f32 = 10.0;

pub struct Physics {
    pub player_x: f32,
    pub ball_pos: Vec2,
    pub ball_dir: Vec2,
    pub boxes: [[bool; BOX_PER_LINE]; BOX_LINE_COUNT],
}

impl Physics {
    pub fn new() -> Self {
        Self {
            player_x: 0.0,
            ball_pos: vec2(30.0, 180.0),
            ball_dir: vec2(-1.0, -1.0).normalize(),
            boxes: [[true; BOX_PER_LINE]; BOX_LINE_COUNT],
        }
    }

    pub fn move_player(&mut self, dt: f32, right: bool) {
        let mut dx = PLAYER_SPEED * dt;
        if !right { dx *= -1.0; }

        self.player_x += dx;
    }

    pub fn update(&mut self, dt: f32) -> bool {
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
            return true;
        }

        for by in 0..BOX_LINE_COUNT {
            for bx in 0..BOX_PER_LINE {
                let box_rect = Self::box_rect(bx, by);

                if !self.boxes[by][bx] {
                    continue;
                }

                if !Self::ball_in_rect(new_ball_pos, box_rect) {
                    continue;
                }

                self.boxes[by][bx] = false;

                if Self::ball_bumped_vertically(self.ball_pos, box_rect) {
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

        false
    }

    pub fn player_rect(&self) -> Rect {
        Rect {
            x: self.player_x,
            y: MAX_Y - PLAYER_HEIGHT - BALL_RADIUS * 1.9,
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
        }
    }

    pub fn box_rect(x: usize, y: usize) -> Rect {
        Rect {
            x: (x as f32) * BOX_WIDTH,
            y: (y as f32) * BOX_HEIGHT,
            w: BOX_WIDTH,
            h: BOX_HEIGHT,
        }
    }

    fn ball_bumped_vertically(pos: Vec2, rect: Rect) -> bool {
        pos.x + BALL_RADIUS >= rect.left() &&
            pos.x - BALL_RADIUS <= rect.right()
    }

    fn ball_in_rect(pos: Vec2, rect: Rect) -> bool {
        pos.x + BALL_RADIUS >= rect.left() &&
        pos.x - BALL_RADIUS <= rect.right() &&
        pos.y + BALL_RADIUS >= rect.top() &&
        pos.y - BALL_RADIUS <= rect.bottom()
    }
}