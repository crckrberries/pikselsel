use crate::frame;
use colored::{self, Colorize};
use std::io::{BufWriter, Write};
use std::net::TcpStream;
use std::{thread, time::Duration};

pub fn send(frames: &Vec<frame::Frame>, host: &str) {
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
        let cmds = frame.commands.join("");
        writer.write(cmds.as_bytes()).unwrap();

        thread::sleep(Duration::new(0, frame.delay * 1000000));
    }
}

pub fn sendloop(frames: Vec<frame::Frame>, host: &str) {
    loop {
        send(&frames, host);
    }
}
