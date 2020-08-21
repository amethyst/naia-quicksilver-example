extern crate quicksilver;

///
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    input::Input,
    run, Result, Settings, Window,
};

pub async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Clear the screen to a blank, white color
    gfx.clear(Color::BLACK);
    // Paint a blue square with a red outline in the center of our screen
    // It should have a top-left of (350, 100) and a size of (150, 100)
    let rect = Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0));
    gfx.fill_rect(&rect, Color::BLUE);
    gfx.stroke_rect(&rect, Color::RED);
    // Send the data to be drawn
    gfx.present(&window)?;
    loop {
        while let Some(_) = input.next_event().await {}
    }

    Ok(())
}