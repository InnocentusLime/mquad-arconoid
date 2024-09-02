#[cfg(target_family = "wasm")]
mod platform {
    use macroquad::prelude::error;

    mod imports {
        extern "C" {
            pub fn done_loading();
            pub fn on_mobile() -> bool;
            pub fn device_fullscreen();
        }
    }

    pub fn done_loading() {
        unsafe { imports::done_loading() }
    }

    pub fn on_mobile() -> bool {
        unsafe { imports::on_mobile() }
    }

    pub fn report_fatal_error(err: anyhow::Error) {
        error!("{}", err);
    }
}

#[cfg(not(target_family = "wasm"))]
mod platform {
    use macroquad::prelude::error;

    pub fn done_loading() { /* Nothing */ }

    pub fn on_mobile() -> bool { false }

    pub fn report_fatal_error(err: anyhow::Error) {
        error!("{}", err);
    }
}

pub use platform::*;