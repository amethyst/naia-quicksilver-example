#[macro_use]
extern crate cfg_if;

extern crate log;

extern crate quicksilver;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {

        use wasm_bindgen::prelude::*;
        use quicksilver::{run};

        mod app;
        use app::{app, get_settings};

        #[wasm_bindgen(start)]
        pub fn main_js() {
            wasm_logger::init(wasm_logger::Config::default());

            run(get_settings(), app)
        }
    } else {}
}
