use image::{DynamicImage, ImageBuffer, Rgba};
use rusttype::{point, Font, Scale};

pub fn render_text(text: String, s: f32, color: (u8, u8, u8)) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let margin = 20;
    let font =
        Font::try_from_bytes(include_bytes!("../fonts/Arial.ttf") as &[u8]).expect("oh nooooooo");
    let scale = Scale::uniform(s);
    let vmetrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font.layout(&text, scale, point(20.0, 20.0)).collect();

    let glyphs_height: u32 = (vmetrics.ascent - vmetrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };
    let mut img = DynamicImage::new_rgba8(glyphs_width + margin, glyphs_height + margin).to_rgba8();

    for x in 1..img.width() {
        for y in 1..img.height() {
            img.put_pixel(x, y, Rgba((32, 32, 32, 255).into()))
        }
    }

    for g in glyphs {
        if let Some(bound) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                if v < 0.001 {
                    return;
                }
                img.put_pixel(
                    x + bound.min.x as u32,
                    y + bound.min.y as u32,
                    Rgba([color.0, color.1, color.2, (v * 255.0) as u8]),
                )
            })
        }
    }
    return img;
}
