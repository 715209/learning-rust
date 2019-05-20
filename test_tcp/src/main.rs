use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    match TcpStream::connect("irc.chat.twitch.tv:6667") {
        Ok(mut stream) => {
            println!("Successfully connected to twitch irc.");

            stream
                .write(b"CAP REQ :twitch.tv/tags twitch.tv/commands\r\n")
                .unwrap();
            stream.write(b"PASS oauth:dfsfdsfdsfdafdsf\r\n").unwrap();
            stream.write(b"NICK 715209\r\n").unwrap();
            stream.write(b"JOIN #715209\r\n").unwrap();

            loop {
                // writer
                // let mut input = String::new();

                // io::stdin()
                //     .read_line(&mut input)
                //     .expect("Failed to read line");

                // stream
                //     .write(format!("PRIVMSG #715209 :{}\r\n", input).as_bytes())
                //     .expect("Failed to write to server");

                // reader
                let mut buffer: Vec<u8> = Vec::new();
                let mut reader = BufReader::new(&stream);

                reader
                    .read_until(b'\n', &mut buffer)
                    .expect("Could not read into buffer.");

                println!(
                    "{}",
                    str::from_utf8(&buffer).expect("Could not write buffer to utf8")
                );
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
