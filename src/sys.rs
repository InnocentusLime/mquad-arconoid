#[cfg(target_family = "wasm")]
mod platform {
    mod imports {
        extern "C" {
            pub fn done_loading();
            pub fn on_mobile() -> bool;
        }
    }

    pub fn done_loading() {
        imports::done_loading()
    }

    pub fn on_mobile() -> bool {
        imports::on_mobile()
    }
}

#[cfg(not(target_family = "wasm"))]
mod platform {
    pub fn done_loading() { /* Nothing */ }

    pub fn on_mobile() -> bool { false }
}

pub use platform::*;