
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
            web_logger::custom_init(web_logger::Config { level: log::Level::Info });

            run(get_settings(), app)
        }
    } else {}
}
