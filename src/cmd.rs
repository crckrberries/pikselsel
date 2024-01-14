use colored::{self, Colorize};
use image::{self, imageops::FilterType::Nearest, ImageBuffer};
mod color;

// this module contains the command generation functions

pub fn wipe(sizex: u32, sizey: u32) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for x in 0..sizex {
        for y in 0..sizey {
            let str = format!("PX {} {} 202020\n", x, y,);
            commands.push(str);
        }
    }
    return commands;
}

pub fn read_image(
    filename: String,
    sizex: u32,
    sizey: u32,
) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::open(&filename) // opens the image
        .unwrap() // unwraps the option
        .resize_exact(sizex, sizey, Nearest) // resizes and stretches the image to specified resolution
        .into_rgba8(); // converts the image into rgb8
    println!(
        "{} {} {} Loaded image {} resized to {}x{}",
        "[".bold().blue(),
        "*".red().bold(),
        "]".bold().blue(),
        filename.bold().red().italic(),
        sizex.to_string().green(),
        sizey.to_string().green()
    );
    return image;
}

pub fn process_image(
    image: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    offsetx: u32,
    offsety: u32,
) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y); // gets the pixel

            let str = format!(
                // creates the command
                "PX {} {} {}\n",
                x + offsetx,
                y + offsety,
                color::Color::hexify_rgb(pixel[0], pixel[1], pixel[2], pixel[3])
            );

            commands.push(str); // pushes to command to list of commands
        }
    }
    println!(
        "{} {} {} Parsed commands",
        "[".bold().blue(),
        "*".bold().red(),
        "]".bold().blue()
    );
    return commands;
}
