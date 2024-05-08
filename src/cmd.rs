use crate::frame;
use colored::{self, Colorize};
use image::{
    self,
    codecs::gif,
    imageops::{self, FilterType::Nearest},
    AnimationDecoder, ImageBuffer,
};
use std::fs::File;

// this module contains the command generation functions

pub fn read_gif(path: &str) -> Vec<image::Frame> {
    let gif = File::open(path).expect("couldn't open gif");
    let dec = gif::GifDecoder::new(gif).expect("that's not a gif!!");
    let frames = dec
        .into_frames()
        .collect_frames()
        .expect("could not decode gif");

    frames
}

pub fn process_gif(
    frames: Vec<image::Frame>,
    size: [u32; 2],
    offset: [u32; 2],
) -> Vec<frame::Frame> {
    let mut frame_list: Vec<frame::Frame> = vec![];
    let mut buffer = image::ImageBuffer::new(size[0], size[1]);
    for frame in frames {
        let delay = frame.delay().numer_denom_ms().0;
        let frame = imageops::resize(frame.buffer(), size[0], size[1], Nearest);
        let cmds = process_image_delta(&frame, &buffer, offset, 10);
        frame_list.push(frame::Frame {
            commands: cmds,
            delay,
        });
        buffer = frame;
    }

    frame_list
}

pub fn wipe(size: [u32; 2]) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for x in 0..size[0] {
        for y in 0..size[1] {
            let str = format!("PX {} {} 202020\n", x, y,);
            commands.push(str);
        }
    }
    commands
}

pub fn read_image(path: &str, size: [u32; 2]) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::open(path) // opens the image
        .unwrap() // unwraps the option
        .resize_exact(size[0], size[1], Nearest) // resizes and stretches the image to specified resolution
        .into_rgba8(); // converts the image into rgb8
    println!(
        "[+] Loaded image {} resized to {}x{}",
        path.bold().red().italic(),
        size[0].to_string().green(),
        size[1].to_string().green()
    );

    image
}

pub fn process_image(
    image: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    offset: [u32; 2],
) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y); // gets the pixel

            let str = format!(
                // creates the command
                "PX {} {} {}\n",
                x + offset[0],
                y + offset[1],
                frame::Color::hexify_rgb(pixel[0], pixel[1], pixel[2], pixel[3])
            );
            commands.push(str); // pushes to command to list of commands
        }
    }

    println!("Processed frame");
    commands
}

pub fn process_image_delta(
    image: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    buffer: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    offset: [u32; 2],
    compression: u8,
) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y); // gets the pixel
            let buf_px = buffer.get_pixel(x, y);

            if pixel[0].abs_diff(buf_px[0]) > compression
                || pixel[1].abs_diff(buf_px[1]) > compression
                || pixel[2].abs_diff(buf_px[2]) > compression
            {
                let str = format!(
                    // creates the command
                    "PX {} {} {}\n",
                    x + offset[0],
                    y + offset[1],
                    frame::Color::hexify_rgb(pixel[0], pixel[1], pixel[2], pixel[3])
                );
                commands.push(str); // pushes to command to list of commands
            }
        }
    }

    println!("Processed frame");
    commands
}

#[cfg(test)]
mod tests {
    use crate::cmd::*;
    use crate::frame::*;

    #[test]
    fn test_still_image() {
        let correct: [&str; 16] = [
            "PX 0 0 160518ff\n",
            "PX 0 1 354542ff\n",
            "PX 0 2 515757ff\n",
            "PX 0 3 181b2cff\n",
            "PX 1 0 17091aff\n",
            "PX 1 1 fefefeff\n",
            "PX 1 2 8baca5ff\n",
            "PX 1 3 0f0d18ff\n",
            "PX 2 0 110918ff\n",
            "PX 2 1 fefefeff\n",
            "PX 2 2 231a2bff\n",
            "PX 2 3 0a0315ff\n",
            "PX 3 0 19131fff\n",
            "PX 3 1 252434ff\n",
            "PX 3 2 191627ff\n",
            "PX 3 3 0e1420ff\n",
        ];

        let img = read_image("src/test/test.jpg", [4, 4]);
        let cmds = process_image(&img, [0, 0]);
        assert_eq!(cmds, correct);
    }

    #[test]
    fn test_delta_compression() {
        let correct = [
            Frame {
                commands: [].to_vec(),
                delay: 100,
            },
            Frame {
                commands: [].to_vec(),
                delay: 100,
            },
            Frame {
                commands: [
                    "PX 0 0 eaeaeaff\n",
                    "PX 0 1 d0d0d0ff\n",
                    "PX 0 2 b6b6b6ff\n",
                    "PX 0 3 9c9c9cff\n",
                    "PX 0 4 828282ff\n",
                    "PX 0 5 686868ff\n",
                    "PX 0 6 4e4e4eff\n",
                    "PX 0 7 2f2f2fff\n",
                    "PX 0 8 1a1a1aff\n",
                ]
                .map(|x| x.to_string())
                .to_vec(),
                delay: 100,
            },
        ];

        let gif = read_gif("src/test/deltatest.gif");
        let cmds = process_gif(gif, [1, 10], [0, 0]);

        assert_eq!(cmds, correct)
    }
}
