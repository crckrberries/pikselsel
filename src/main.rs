use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, BufWriter, Write};
use std::net::TcpStream;
mod cmd;

fn wipe(sizex: u32, sizey: u32) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for x in 0..sizex {
        for y in 0..sizey {
            let str = format!("PX {} {} 202020\n", x, y,);
            commands.push(str);
        }
    }
    return commands;
}

fn main() -> io::Result<()> {
    let host: String = "127.0.0.1:1234".to_string(); // replace with ip and port you need
    let mode = "i";

    let sizex: u32 = 1280; // size of the image
    let sizey: u32 = 720;

    let mut xoff: u32 = 0; // offset from the top left corner
    let mut yoff: u32 = 0;

    let looping: bool = true; // whether to loop the draw cycle or not
    let shuffle = true; // whether to randomize the sequence of the commands, creating a dithering effect
    let tile = false;

    let mut stream = BufWriter::new(TcpStream::connect(host)?);
    let img = cmd::read_image("images/horse.jpg".to_string(), sizex, sizey); // reads image (check function def for details)
    while looping {
        let mut cmds: Vec<String>;
        if mode == "w" {
            cmds = wipe(sizex, sizey); // wipes screen
        } else {
            cmds = cmd::process_image(&img, xoff, yoff); // processes image, generating commands
        }

        if tile {
            // tiling function
            if xoff >= 1100 {
                // horizontal distance to tile to
                yoff += sizex;
                xoff = 0;
            } else if yoff >= 600 {
                // vertical distance to tile to
                yoff = 0;
            } else {
                xoff += sizey;
            }
        }

        if shuffle {
            cmds.shuffle(&mut thread_rng());
        }

        for cmd in cmds {
            stream.write_all(cmd.as_bytes()).unwrap(); // send commands to pixelflut server
        }
    }

    stream.flush().unwrap(); // flushes the stream, sending everything not sent

    Ok(())
}
