#[macro_use]
extern crate cfg_if;

extern crate log;

extern crate quicksilver;

mod app;

use app::app;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    // Uncomment the line below to enable logging. You don't need it if something
    // else (e.g. quicksilver) is logging for you
    simple_logger::init_with_level(log::Level::Info).expect("A logger was already initialized");

    run(
        Settings {
            title: "naia Quicksilver examplE",
            ..Settings::default()
        },
        app,
    );
}