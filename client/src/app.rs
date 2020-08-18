
use quicksilver::{
    graphics::{Color, Graphics},
    geom::{Rectangle, Vector},
    Window,
    Result,
    Input,
};

pub async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
//    let image = Image::load(&gfx, "head.png").await?;
//    gfx.clear(Color::WHITE);
//    // Draw the image with the top-left at (100, 100)
//    let region = Rectangle::new(Vector::new(100.0, 100.0), image.size());
//    gfx.draw_image(&image, region);
//    gfx.present(&window)?;

    // Clear the screen to a blank, white color
    gfx.clear(Color::WHITE);
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
}