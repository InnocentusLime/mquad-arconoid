// uncommenting this caues a panic. we need to investigate
// #[no_mangle]
// pub extern "C" fn mquad_arcanoid_crate_version() -> u32 {
//     let major = dbg!(env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap());
//     let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
//     let patch = dbg!(env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap());

//     (major << 24) + (minor << 16) + patch
// }

#[cfg(target_family = "wasm")]
mod platform {
    use macroquad::prelude::error;

    mod imports {
        extern "C" {
            pub fn app_done_loading();
            pub fn app_is_on_mobile() -> bool;
            pub fn app_get_orientation() -> f32;
        }
    }

    pub fn done_loading() {
        unsafe { imports::app_done_loading() }
    }

    pub fn on_mobile() -> bool {
        unsafe { imports::app_is_on_mobile() }
    }

    pub fn report_fatal_error(err: anyhow::Error) {
        error!("{}", err);
    }

    pub fn get_orientation() -> f32 {
        unsafe { imports::app_get_orientation() }
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

    pub fn get_orientation() -> f32 {
        0.0
    }
}

pub use platform::*;