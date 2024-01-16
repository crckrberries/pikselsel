use colored::{self, Colorize};
use std::io::{BufWriter, Write};
use std::{thread, time::Duration};
use std::net::TcpStream;
use crate::frame;

pub fn send(frames: Vec<frame::Frame>, host: &str) {
    let stream: TcpStream = TcpStream::connect(host).unwrap();
    let mut writer: BufWriter<&TcpStream> = BufWriter::new(&stream);
    println!(
        "{} {} {} Sending commands to {}",
        "[".bold().blue(),
        "*".red().bold(),
        "]".bold().blue(),
        host.bold().red().italic(),
    );
    for frame in frames {
        for cmd in frame.commands {
            writer.write(cmd.as_bytes()).unwrap();
        }

        thread::sleep(Duration::new(0, 100 * 1000000));
    }
}

pub fn sendloop(frames: Vec<frame::Frame>, host: &str) {
    loop {
        send(frames.clone(), host);
    }
}
