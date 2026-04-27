use hyprland::event_listener;
use iced::futures::StreamExt;


use hyprland::event_listener::Event;

pub async fn run() {
   let mut listener = event_listener::EventStream::new();

   loop {
      match listener.next().await.unwrap().unwrap() {
         Event::ActiveMonitorChanged(event) => {
            println!("workspace changed to: {:?}", event.workspace_name);
            println!("monitor changed to: {}", event.monitor_name);
         },
         Event::WorkspaceChanged(event) => {
            println!("workspace changed to: {:?}", event.name);
         },
         _ => (),
      }
   }


}
