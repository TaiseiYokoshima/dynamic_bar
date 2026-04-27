use std::fs;
use tokio::net::UnixDatagram;

pub async fn run() -> std::io::Result<()> {
   let path = "/tmp/raw.sock";

   // remove stale socket file
   let _ = fs::remove_file(path);

   let socket = UnixDatagram::bind(path)?;
   println!("listening on {}", path);

   let mut buf = vec![0u8; 1024];

   loop {
      let (len, addr) = socket.recv_from(&mut buf).await?;
      let string: &str = std::str::from_utf8(&buf[..len]).unwrap();
      println!("got {} bytes: {:?}", len, string);
   }
}
