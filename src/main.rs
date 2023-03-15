use std::{env, io, net::UdpSocket};

const MAX_LENGTH: usize = 20;

fn listen(expected: &[u8]) -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:34255")?;
    println!("Listening on 0.0.0.0:34255");
    let mut buf = [0; MAX_LENGTH];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        if buf.starts_with(expected) {
            println!("Got needle from {} - responding", src);
            socket.send_to("helloworld".as_bytes(), &src)?;
            socket.send_to("helloworld".as_bytes(), &src)?;
            socket.send_to("helloworld".as_bytes(), &src)?;
        } else {
            println!("Got junk from {}", src);
        }
    }
}

fn main() -> io::Result<()> {
    let expected = env::var("UDP_FLARE_INPUT").expect("UDP_FLARE_INPUT is required");
    if expected.len() > MAX_LENGTH {
        eprintln!("UDP_FLARE_INPUT is too long");
        return Err(io::Error::from_raw_os_error(22));
    }
    listen(expected.as_bytes())
}
