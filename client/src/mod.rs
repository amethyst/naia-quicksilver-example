
extern crate quicksilver;

use quicksilver::lifecycle::{ run, Settings };

mod app;

pub fn main() {
    run(
        Settings {
            title: "Cyberlith",
            ..Settings::default()
        },
        app::app,
    );
}