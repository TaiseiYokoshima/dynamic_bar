use evdev::{Device, KeyCode};
use std::fs::File;
use tokio::io::unix::AsyncFd;

pub async fn run() -> std::io::Result<()> {
   let file = File::open("/dev/input/event0")?;
   let file = std::os::fd::OwnedFd::from(file);
   let device = Device::from_fd(file)?;

   // Wrap the device FD for async readiness
   let mut async_fd = AsyncFd::new(device)?;

   println!("Listening (async)...");

   loop {
      let mut guard = async_fd.readable_mut().await?;

      let dev = guard.get_inner_mut();

      let events = dev.fetch_events().unwrap();

      for event in events {
         let event = match event.destructure() {
            evdev::EventSummary::Key(_, KeyCode::KEY_LEFTMETA, x) => x,
            _ => continue,
         };

         match event {
            1 => println!("SUPER pressed"),
            0 => println!("SUPER released"),
            2 => println!("SUPER held"),
            x => eprintln!("error got something else: {x}"),
         };
      };
      guard.clear_ready();
   }


}
