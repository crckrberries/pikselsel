use clap::Parser;
use rand::seq::SliceRandom;
use rand::*;
use std::io::{self, ErrorKind};
mod cmd;
mod sender;
mod text;

#[derive(Default, Parser, Debug)]
struct Cli {
    /// the mode of the program - you can currently pick between wipe, image, and text
    mode: String,
    file: String,
    host: String,
    size: String,

    #[arg(long, short = 's', value_name = "SHUFFLE", default_value_t = false)]
    shuffle: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let host: String = cli.host; // replace with ip and port you need
    let mode: String = cli.mode;

    let filename: String = cli.file;
    let mut size = cli.size.split('x');
    let sizex: u32 = size.next().unwrap().parse().unwrap();
    let sizey: u32 = size.next().unwrap().parse().unwrap();

    let xoff: u32 = 0; // offset from the top left corner
    let yoff: u32 = 0;

    let looping: bool = false; // whether to loop the draw cycle or not
    let shuffle: bool = cli.shuffle.clone(); // whether to randomize the sequence of the commands, creating a dithering effect
                                     // let tile: bool = false;

    println!("Mode: {}; Shuffle: {}", mode, shuffle);

    let mut cmds: Vec<String>;
    if mode == "wipe" {
        cmds = cmd::wipe(sizex, sizey); // wipes screen
    } else if mode == "text" {
        let txt = "balls 2".to_string();
        let size = 30.0;
        let color = (255, 255, 255); // if it goes above 30 it freaks out :3
        cmds = cmd::process_image(&text::render_text(txt, size, color), xoff, yoff)
    } else if mode == "img" {
        let img = cmd::read_image(filename.clone(), sizex, sizey); // reads image (check function def for details)
        cmds = cmd::process_image(&img, xoff, yoff); // processes image, generating commands
    } else {
        println!("incorrect option {}", mode);
        return Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            "wrong input lmao",
        ));
    }

    if shuffle {
        cmds.shuffle(&mut thread_rng());
    }

    match looping {
        true => sender::sendloop(cmds, &host),

        false => {
            sender::send(cmds, &host);
        }
    }

    // writer.flush().unwrap();

    Ok(())
}
