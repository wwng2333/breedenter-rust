use std::io;
use std::io::Read;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:37540")?;
    socket.set_broadcast(true)?;
    socket.set_nonblocking(true).unwrap();
    println!("Sending abort command to breed per 500ms.");
    loop {
        socket.send_to("BREED:ABORT".as_bytes(), "255.255.255.255:37541")?;
        let mut buf = [0; 14];
        match socket.recv_from(&mut buf) {
            Ok(_) => {
                println!("Received pong from breed, starting browser.");
                open::that("http://192.168.1.1").unwrap();
                println!("Press ENTER to continue...");
                let buffer = &mut [0u8];
                io::stdin().read_exact(buffer).unwrap();
                return Ok(());
            }
            Err(_) => {
                thread::sleep(Duration::from_secs_f32(0.5));
                continue;
            }
        }
    }
}
