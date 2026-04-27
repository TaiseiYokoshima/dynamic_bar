use iced::widget::{button, column, row, text, text_input};
use iced::{Alignment, Color, Element, Event, Length, Task as Command, event};
use iced_layershell::application;
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::to_layer_message;

pub fn run() -> Result<(), iced_layershell::Error> {
   let binded_output_name = std::env::args().nth(1);
   let start_mode = match binded_output_name {
      Some(output) => StartMode::TargetScreen(output),
      None => StartMode::Active,
   };

   application(|| Counter::default(), namespace, update, view)
      .style(style)
      .subscription(subscription)
      .settings(Settings {
         layer_settings: LayerShellSettings {
            size: None,
            exclusive_zone: -1,
            anchor: Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right,
            start_mode,
            layer: Layer::Overlay, 
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::None,
            ..Default::default()
         },
         ..Default::default()
      })
      .run()
}

#[derive(Default)]
struct Counter {
   value: i32,
   text: String,
}

#[derive(Debug, Clone, Copy)]
enum WindowDirection {
   Top,
   Left,
   Right,
   Bottom,
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
   IncrementPressed,
   DecrementPressed,
   TextInput(String),
   Direction(WindowDirection),
   IcedEvent(Event),
}

fn namespace() -> String {
   String::from("dynamic_bar")
}

fn subscription(_: &Counter) -> iced::Subscription<Message> {
   event::listen().map(Message::IcedEvent)
}

fn update(counter: &mut Counter, message: Message) -> Command<Message> {
   Command::none()
}

fn view(counter: &Counter) -> Element<Message> {
   iced::widget::column![].width(iced::Length::Fill).height(iced::Length::Fill).into()
}

fn style(_counter: &Counter, theme: &iced::Theme) -> iced::theme::Style {
   use iced::theme::Style;
   Style {
      background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
      text_color: Color::TRANSPARENT,
   }
}

