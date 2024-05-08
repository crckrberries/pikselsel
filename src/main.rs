use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
use rand::*;
use std::io::{self};

mod cmd;
mod frame;
mod sender;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// hostname and port (example: pixelflut.organs.trade:1234)
    host: String,
    /// offset from the top left of the canvas, in pixels
    offset: String,

    #[command(subcommand)]
    /// the command to run
    cmd: Commands,

    size: String,

    /// whether to shuffle the pixel order or not
    #[arg(long, short = 's', default_value_t = false)]
    shuffle: bool,
    /// whether to loop the drawing cycle or not
    #[arg(long, short = 'l', default_value_t = false)]
    looping: bool,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// send an image to the pixelflut canvas
    Img {
        /// path to the image
        path: String,
    },

    /// wipe the pixelflut canvas
    Wipe {},

    /// send an animated gif to the pixelflut canvas
    Gif {
        /// path to the gif
        path: String,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let host: String = cli.host; // replace with ip and port you need

    let offset: [u32; 2] = cli
        .offset
        .split('x')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();

    let size: [u32; 2] = cli
        .size
        .split('x')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();

    let mut frames: Vec<frame::Frame> = vec![];
    match cli.cmd {
        Commands::Img { path } => {
            let img = cmd::read_image(&path, size);
            let cmds = cmd::process_image(&img, offset); // processes image, generating commands
            frames.push(frame::Frame {
                commands: cmds,
                delay: 0,
            })
        }

        Commands::Wipe {} => {
            let cmds = cmd::wipe(size); // wipes screen
            frames.push(frame::Frame {
                commands: cmds,
                delay: 0,
            })
        }

        Commands::Gif { path } => {
            let img = cmd::read_gif(&path);
            frames = cmd::process_gif(img, size, offset);
        }
    };

    if cli.shuffle {
        for frame in &mut frames {
            frame.commands.shuffle(&mut thread_rng());
        }
    }

    match cli.looping {
        true => sender::send_loop(frames, &host),

        false => sender::send(&frames, &host),
    }

    Ok(())
}
