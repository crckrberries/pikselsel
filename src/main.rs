use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, BufWriter, Write};
use std::net::TcpStream;
mod cmd;

fn main() -> io::Result<()> {
    let host: String = "127.0.0.1:1234".to_string(); // replace with ip and port you need
    let mode = "i";

    let filename: String = String::from("images/uni.jpg");
    let sizex: u32 = 500; // size of the image
    let sizey: u32 = 500;

    let xoff: u32 = 390; // offset from the top left corner
    let yoff: u32 = 100;

    // let looping: bool = false; // whether to loop the draw cycle or not
    let shuffle: bool = false; // whether to randomize the sequence of the commands, creating a dithering effect
                               // let tile: bool = false;

    let mut stream = BufWriter::new(TcpStream::connect(host)?);
    let img = cmd::read_image(filename, sizex, sizey); // reads image (check function def for details)
                                                       // while looping {
    let mut cmds: Vec<String>;
    if mode == "w" {
        cmds = cmd::wipe(sizex, sizey); // wipes screen
    } else {
        cmds = cmd::process_image(&img, xoff, yoff); // processes image, generating commands
    }

    // if tile {
    //     // tiling function
    //     if xoff >= 1200 {
    //         // horizontal distance to tile to
    //         yoff += sizex;
    //         xoff = 0;
    //     } else if yoff >= 720 {
    //         // vertical distance to tile to
    //         yoff = 0;
    //     } else {
    //         xoff += sizey;
    //     }
    // }

    if shuffle {
        cmds.shuffle(&mut thread_rng());
    }

    for cmd in &cmds {
        stream.write_all(cmd.as_bytes()).unwrap(); // send commands to pixelflut server
    }

    stream.flush().unwrap(); // flushes the stream, sending everything not sent

    Ok(())
}
