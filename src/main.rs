use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, BufWriter, Write};
use std::net::TcpStream;
mod cmd;
mod text;

fn main() -> io::Result<()> {
    let host: String = "pixelflut.uwu.industries:1234".to_string(); // replace with ip and port you need
    let mode = "i";

    let filename: String = String::from("images/binky.jpg");
    let sizex: u32 = 100; // size of the image
    let sizey: u32 = 100;

    let xoff: u32 = 580; // offset from the top left corner
    let yoff: u32 = 500;

    // let looping: bool = false; // whether to loop the draw cycle or not
    let shuffle: bool = true; // whether to randomize the sequence of the commands, creating a dithering effect
                              // let tile: bool = false;

    let stream = TcpStream::connect(host)?;
    // while looping {
    let mut cmds: Vec<String>;
    if mode == "w" {
        cmds = cmd::wipe(sizex, sizey); // wipes screen
    } else if mode == "t" {
        let txt = "balls 2".to_string();
        let size = 30.0;
        let color = (255, 255, 255); // if it goes above 30 it freaks out :3
        cmds = cmd::process_image(&text::render_text(txt, size, color), xoff, yoff)
    } else {
        let img = cmd::read_image(filename, sizex, sizey); // reads image (check function def for details)
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

    // for i in 0..1000 {
    //     let pthread_cmds = Arc::new(cmds.clone());
    //     let pthread_stream = Arc::clone(&stream);
    //     thread::spawn(move || {
    //         dbg!(i);
    //         let mut writer = BufWriter::new(pthread_stream.as_ref());

    //         // thread::sleep(Duration::new(10, 0))
    //     });
    // }

    let mut writer = BufWriter::new(stream);

    // let chunks = cmds.chunks(cmds.len() / 4);

    loop {
        for cmd in &cmds {
            writer.write(cmd.as_bytes()).unwrap(); // send commands to pixelflut server
        }
    }

    writer.flush().unwrap();

    Ok(())
}
