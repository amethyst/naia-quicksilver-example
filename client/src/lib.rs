
#[macro_use]
extern crate cfg_if;

extern crate log;

extern crate quicksilver;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {

        use wasm_bindgen::prelude::*;
        use quicksilver::{run, Settings};

        mod app;
        use app::app;

        #[wasm_bindgen(start)]
        pub fn main_js() {
            web_logger::custom_init(web_logger::Config { level: log::Level::Info });

            run(Settings::default(), app::app)
        }
    } else {}
}
