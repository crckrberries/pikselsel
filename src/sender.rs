use std::io::{BufWriter, Write};
use std::net::TcpStream;

pub fn send(commands: Vec<String>, host: &str) {
    let stream: TcpStream = TcpStream::connect(host).unwrap();
    let mut writer: BufWriter<&TcpStream> = BufWriter::new(&stream);

    for cmd in commands {
        writer.write(cmd.as_bytes()).unwrap();
    }
}

pub fn sendloop(commands: Vec<String>, host: &str) {
    loop {
        send(commands.clone(), host)
    }
}