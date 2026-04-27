use iced::{Color, Program, Settings, theme::Style, window::{self, settings::PlatformSpecific}};


#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn update(_: &mut u64, message: Message) {}

fn view(value: &u64) -> iced::widget::Column<Message> {
   use iced::widget::{text, button};


   iced::widget::column![
      text(value),
      button("+").on_press(Message::Increment),
   ]

}

pub fn run() -> iced::Result {
   let mut app = iced::application(|| u64::default(), update, view);
   let app = app.settings(Settings {
      id: Some(String::from("test")),
      ..Default::default()
   });

   let app = app.window(window::Settings { 
      platform_specific: PlatformSpecific { application_id: "test my bar".into(), ..Default::default() },
      transparent: true,
      ..Default::default()
   });

   let app = app.title("test").style(|test, what| {
      Style {
         background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
         text_color: Color::WHITE,
      }
   });

   app.run()
}
