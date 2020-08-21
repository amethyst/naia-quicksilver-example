#[macro_use]
extern crate cfg_if;
extern crate log;

extern crate quicksilver;
use quicksilver::run;

mod app;
use app::{app, get_settings};

fn main() {
    // Uncomment the line below to enable logging. You don't need it if something
    // else (e.g. quicksilver) is logging for you
    simple_logger::init_with_level(log::Level::Info).expect("A logger was already initialized");

    run(get_settings(), app)
}