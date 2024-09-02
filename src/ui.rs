use macroquad::prelude::*;
use crate::{sys::*, GameState};

const FONT_SCALE: f32 = 1.3;
const FONT_SIZE: u16 = 32;
const PADDLE_BUTTON_WIDTH: f32 = 128.0 * 1.5;

static WIN_TEXT: &'static str = "Congratulations!";
static START_TEXT: &'static str = "SPACE to start";
static GAMEOVER_TEXT: &'static str = "Game Over";
static PAUSE_TEXT: &'static str = "Paused";

#[derive(Clone, Copy, Debug)]
pub struct InGameUiModel {
    state: GameState,
    left_movement_down: bool,
    right_movement_down: bool,
    confirmation_detected: bool,
    pause_requested: bool,
    fullscreen_toggle_requested: bool,
}

impl InGameUiModel {
    pub fn move_left(&self) -> bool {
        self.left_movement_down
    }

    pub fn move_right(&self) -> bool {
        self.right_movement_down
    }

    pub fn confirmation_detected(&self) -> bool {
        self.confirmation_detected
    }

    pub fn pause_requested(&self) -> bool {
        self.pause_requested
    }

    pub fn fullscreen_toggle_requested(&self) -> bool {
        self.fullscreen_toggle_requested
    }
}

pub struct Ui {
    oegnek: Font,
}

impl Ui {
    pub async fn new() -> Self {
        Self {
            oegnek: load_ttf_font("assets/oegnek.ttf").await.unwrap(),
        }
    }

    pub fn update(&self, state: GameState) -> InGameUiModel {
        let (mx, my) = mouse_position();
        let left_movement_down =
            is_key_down(KeyCode::A) ||
            is_key_down(KeyCode::Left) ||
            (Self::move_left_button_rect().contains(vec2(mx, my)) &&
             is_mouse_button_down(MouseButton::Left) &&
             on_mobile());
        let right_movement_down =
            is_key_down(KeyCode::D) ||
            is_key_down(KeyCode::Right) ||
            (Self::move_right_button_rect().contains(vec2(mx, my)) &&
             is_mouse_button_down(MouseButton::Left) &&
             on_mobile());
        let confirmation_detected =
            is_key_pressed(KeyCode::Space) ||
            is_mouse_button_pressed(MouseButton::Left);
        let pause_requested =
            is_key_pressed(KeyCode::Escape);
        let fullscreen_toggle_requested =
            is_key_pressed(KeyCode::F11);

        InGameUiModel {
            state,
            left_movement_down,
            right_movement_down,
            confirmation_detected,
            pause_requested,
            fullscreen_toggle_requested,
        }
    }

    pub fn draw(&self, model: InGameUiModel) {
        set_default_camera();

        if on_mobile() && model.state == GameState::Active {
            draw_rectangle(
                Self::move_left_button_rect().x,
                Self::move_left_button_rect().y,
                Self::move_left_button_rect().w,
                Self::move_left_button_rect().h,
                if model.move_left() { WHITE }
                else { Color::from_hex(0xDDFBFF) }
            );
            draw_rectangle(
                Self::move_right_button_rect().x,
                Self::move_right_button_rect().y,
                Self::move_right_button_rect().w,
                Self::move_right_button_rect().h,
                if model.move_right() { WHITE }
                else { Color::from_hex(0xDDFBFF) }
            );
        }

        match model.state {
            GameState::Start => self.draw_announcement_text(true, START_TEXT),
            GameState::GameOver => self.draw_announcement_text(true, GAMEOVER_TEXT),
            GameState::Win => self.draw_announcement_text(false, WIN_TEXT),
            GameState::Paused => self.draw_announcement_text(true, PAUSE_TEXT),
            _ => (),
        }
    }

    fn move_left_button_rect() -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            w: PADDLE_BUTTON_WIDTH,
            h: screen_height(),
        }
    }

    fn move_right_button_rect() -> Rect {
        Rect {
            x: screen_width() - PADDLE_BUTTON_WIDTH,
            y: 0.0,
            w: PADDLE_BUTTON_WIDTH,
            h: screen_height(),
        }
    }

    fn draw_announcement_text(&self, backdrop: bool, text: &str) {
        if backdrop {
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
        }

        let center = get_text_center(
            text,
            Some(&self.oegnek),
            FONT_SIZE,
            FONT_SCALE,
            0.0
        );
        draw_text_ex(
            text,
            screen_width() / 2.0 - center.x,
            screen_height() / 2.0 - center.y,
            TextParams {
                font: Some(&self.oegnek),
                font_size: FONT_SIZE,
                color: Color::from_hex(0xDDFBFF),
                font_scale: FONT_SCALE,
                ..Default::default()
            }
        );
    }
}