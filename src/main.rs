use rand::seq::SliceRandom;
use rand::*;
use std::io::{self};
mod cmd;
mod text;
mod sender;

fn main() -> io::Result<()> {
    let host: String = "pixelflut.uwu.industries:1234".to_string(); // replace with ip and port you need
    let mode = "i";

    let filename: String = String::from("images/sakaparent.png");
    let sizex: u32 = 1280; // size of the image
    let sizey: u32 = 720;

    let xoff: u32 = 0; // offset from the top left corner
    let yoff: u32 = 0;

    // let looping: bool = false; // whether to loop the draw cycle or not
    let shuffle: bool = true; // whether to randomize the sequence of the commands, creating a dithering effect
                              // let tile: bool = false;

    let mut cmds: Vec<String>;
    if mode == "w" {
        cmds = cmd::wipe(sizex, sizey); // wipes screen
    } else if mode == "t" {
        let txt = "balls 2".to_string();
        let size = 30.0;
        let color = (255, 255, 255); // if it goes above 30 it freaks out :3
        cmds = cmd::process_image(&text::render_text(txt, size, color), xoff, yoff)
    } else {
        let img = cmd::read_image(filename.clone(), sizex, sizey); // reads image (check function def for details)
        cmds = cmd::process_image(&img, xoff, yoff); // processes image, generating commands
    }

    if shuffle {
        cmds.shuffle(&mut thread_rng());
    }

    // let chunks = cmds.chunks(cmds.len() / 4);

    sender::send(cmds, &host);

    // writer.flush().unwrap();

    Ok(())
}
