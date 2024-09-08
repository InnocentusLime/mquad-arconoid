use macroquad::prelude::*;
use crate::{sys::*, GameState};

const FONT_SCALE: f32 = 1.0;
const FONT_SIZE: u16 = 32;
const PADDLE_BUTTON_WIDTH: f32 = 128.0;

static WIN_TEXT: &'static str = "Congratulations!";
static START_TEXT: &'static str = "SPACE to start";
static GAMEOVER_TEXT: &'static str = "Game Over";
static PAUSE_TEXT: &'static str = "Paused";
static ORIENTATION_TEXT: &'static str = "Please put your device into landscape orientation";

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
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            oegnek: load_ttf_font("assets/oegnek.ttf").await?,
        })
    }

    pub fn update(&self, state: GameState) -> InGameUiModel {
        let view_height = (FONT_SIZE as f32) * 20.0;
        let view_rect = Rect {
            x: 0.0,
            y: 0.0,
            w: view_height * (screen_width() / screen_height()),
            h: view_height,
        };
        let mut cam = Camera2D::from_display_rect(view_rect);
        cam.zoom.y *= -1.0;
        let (mx, my) = mouse_position();
        let Vec2 { x: mx, y: my } = cam.screen_to_world(vec2(mx, my));

        let left_movement_down =
            is_key_down(KeyCode::A) ||
            is_key_down(KeyCode::Left) ||
            (Self::move_left_button_rect(view_rect).contains(vec2(mx, my)) &&
             is_mouse_button_down(MouseButton::Left) &&
             on_mobile());
        let right_movement_down =
            is_key_down(KeyCode::D) ||
            is_key_down(KeyCode::Right) ||
            (Self::move_right_button_rect(view_rect).contains(vec2(mx, my)) &&
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
        let view_height = (FONT_SIZE as f32) * 20.0;
        let view_rect = Rect {
            x: 0.0,
            y: 0.0,
            w: view_height * (screen_width() / screen_height()),
            h: view_height,
        };
        let mut cam = Camera2D::from_display_rect(view_rect);
        cam.zoom.y *= -1.0;
        set_camera(&cam);

        if on_mobile() && model.state == GameState::Active {
            draw_rectangle(
                Self::move_left_button_rect(view_rect).x,
                Self::move_left_button_rect(view_rect).y,
                Self::move_left_button_rect(view_rect).w,
                Self::move_left_button_rect(view_rect).h,
                if model.move_left() { WHITE }
                else { Color::from_hex(0xDDFBFF) }
            );
            draw_rectangle(
                Self::move_right_button_rect(view_rect).x,
                Self::move_right_button_rect(view_rect).y,
                Self::move_right_button_rect(view_rect).w,
                Self::move_right_button_rect(view_rect).h,
                if model.move_right() { WHITE }
                else { Color::from_hex(0xDDFBFF) }
            );
        }

        match model.state {
            GameState::Start => self.draw_announcement_text(true, START_TEXT, view_rect),
            GameState::GameOver => self.draw_announcement_text(true, GAMEOVER_TEXT, view_rect),
            GameState::Win => self.draw_announcement_text(false, WIN_TEXT, view_rect),
            GameState::Paused => self.draw_announcement_text(true, PAUSE_TEXT, view_rect),
            GameState::PleaseRotate => self.draw_announcement_text(true, ORIENTATION_TEXT, view_rect),
            _ => (),
        }
    }

    fn move_left_button_rect(view_rect: Rect) -> Rect {
        Rect {
            x: view_rect.left(),
            y: view_rect.top(),
            w: PADDLE_BUTTON_WIDTH,
            h: view_rect.h,
        }
    }

    fn move_right_button_rect(view_rect: Rect) -> Rect {
        Rect {
            x: view_rect.right() - PADDLE_BUTTON_WIDTH,
            y: view_rect.top(),
            w: PADDLE_BUTTON_WIDTH,
            h: view_rect.h,
        }
    }

    fn draw_announcement_text(&self, backdrop: bool, text: &str, view_rect: Rect) {
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
            view_rect.left() + view_rect.w / 2.0 - center.x,
            view_rect.top() + view_rect.h / 2.0 - center.y,
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