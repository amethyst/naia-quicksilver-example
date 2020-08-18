extern crate quicksilver;

use quicksilver::{
    graphics::{Color, Graphics, Image},
    geom::{Rectangle, Vector},
    Window,
    Result,
    Input,
};

pub async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let image = Image::load(&gfx, "head.png").await?;
    gfx.clear(Color::WHITE);
    // Draw the image with the top-left at (100, 100)
    let region = Rectangle::new(Vector::new(100.0, 100.0), image.size());
    gfx.draw_image(&image, region);
    gfx.present(&window)?;

    loop {
        while let Some(_) = input.next_event().await {}
    }
}