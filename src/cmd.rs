use std::fs::File;
use crate::frame;
use colored::{self, Colorize};
use image::{
    self,
    codecs::gif,
    imageops::{self, FilterType::Nearest},
    AnimationDecoder, ImageBuffer,
};

// this module contains the command generation functions

pub fn read_gif(filename: String) -> Vec<image::Frame> {
    let gif = File::open(filename).expect("couldnt open gif");
    let dec = gif::GifDecoder::new(gif).expect("thats not a gif!!");
    let frames = dec
        .into_frames()
        .collect_frames()
        .expect("could not decode gif");

    return frames;
}

pub fn process_gif(
    frames: Vec<image::Frame>,
    sizex: u32,
    sizey: u32,
    xoff: u32,
    yoff: u32
) -> Vec<frame::Frame> {
    let mut framelist: Vec<frame::Frame> = vec![];
    let mut buffer = image::ImageBuffer::new(sizex, sizey);
    for frame in frames {
        let delay = frame.delay().numer_denom_ms().0;
        let frame = imageops::resize(frame.buffer(), sizex, sizey, Nearest);
        let cmds = process_image_delta(&frame, &buffer, xoff, yoff, 15);
        framelist.push(frame::Frame {
            commands: cmds,
            delay,
        });
        buffer = frame;
    }

    return framelist;
}

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
                frame::Color::hexify_rgb(pixel[0], pixel[1], pixel[2], pixel[3])
            );
            commands.push(str); // pushes to command to list of commands
        }
    }
    // if shuffle {
    //     commands.shuffle(&mut thread_rng())
    // }
    println!("Processed frame");
    return commands;
}

pub fn process_image_delta(
    image: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    buffer: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    offsetx: u32,
    offsety: u32,
    compression: u8
) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y); // gets the pixel
            let bufpx = buffer.get_pixel(x, y);

            if pixel[0].abs_diff(bufpx[0]) > compression
                || pixel[1].abs_diff(bufpx[1]) > compression
                || pixel[2].abs_diff(bufpx[2]) > compression
            {
                let str = format!(
                    // creates the command
                    "PX {} {} {}\n",
                    x + offsetx,
                    y + offsety,
                    frame::Color::hexify_rgb(pixel[0], pixel[1], pixel[2], pixel[3])
                );
                commands.push(str); // pushes to command to list of commands
            }
        }
    }

    println!("Processed frame");
    return commands;
}
