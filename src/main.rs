use std::{env, io, net::UdpSocket};

const LISTEN_ADDRESS: &str = "0.0.0.0:34255";
const MAX_INPUT_LENGTH: usize = 20;

fn listen(expected: &[u8], reply: &[u8]) -> io::Result<()> {
    let socket = UdpSocket::bind(LISTEN_ADDRESS)?;
    println!("Listening on {}", LISTEN_ADDRESS);
    let mut buf = [0; MAX_INPUT_LENGTH];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        if buf.starts_with(expected) {
            println!("Got needle from {} - responding", src);
            socket.send_to(reply, &src)?;
        } else {
            println!("Got junk from {}", src);
        }
    }
}

fn main() -> io::Result<()> {
    let expected = env::var("UDP_FLARE_INPUT").expect("UDP_FLARE_INPUT is required");
    if expected.len() > MAX_INPUT_LENGTH {
        eprintln!("UDP_FLARE_INPUT is too long");
        return Err(io::Error::from_raw_os_error(22));
    }
    let reply = env::var("UDP_FLARE_REPLY").unwrap_or(String::from("helloworld"));
    listen(expected.as_bytes(), reply.as_bytes())
}
