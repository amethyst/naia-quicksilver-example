extern crate log;

use simple_logger::init_with_level;

extern crate quicksilver;
use quicksilver::run;

mod app;
use app::{app, get_settings};

fn main() {
    init_with_level(log::Level::Info).expect("A logger was already initialized");

    run(get_settings(), app)
}
