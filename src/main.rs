use std::{env, io, net::UdpSocket};

fn listen(expected: &[u8]) -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:34255")?;
    let mut buf = [0; 20];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        println!("{:#?}", buf);
        if buf.starts_with(expected) {
            socket.send_to("helloworld".as_bytes(), &src)?;
            socket.send_to("helloworld".as_bytes(), &src)?;
            socket.send_to("helloworld".as_bytes(), &src)?;
        }
    }
}

fn main() -> io::Result<()> {
    let expected = env::var("UDP_FLARE_INPUT").expect("UDP_FLARE_INPUT is required");
    listen(expected.as_bytes())
}
