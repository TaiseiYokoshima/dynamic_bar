use iced::futures::stream::BoxStream;
use iced::widget::{button, column, row, text, text_input};
use iced::{Alignment, Color, Element, Event, Length, Task, event};
use iced_layershell::application;
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::to_layer_message;

use iced::advanced::subscription::Recipe;


use tokio_util::sync::CancellationToken;


use async_stream;

use std::hash::Hash;

use std::sync::Arc;
pub async fn run(
   token: CancellationToken,
   monitor: Option<String>,
) -> Result<(), iced_layershell::Error> {
   let start_mode = match monitor {
      Some(output) => StartMode::TargetScreen(output),
      None => StartMode::Active,
   };


   let token = Arc::new(token);

   let handle = tokio::task::spawn_blocking(|| {
      application(
         move || App {
            token: token.clone(),
         },
         namespace,
         update,
         view,
      )
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
   });

   handle.await.unwrap()
}

// #[derive(Default)]
struct App {
   token: Arc<CancellationToken>,
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
   Shutdown,
   IncrementPressed,
   DecrementPressed,
   TextInput(String),
   IcedEvent(Event),
}

struct ShutdownRecipe {
   token: Arc<CancellationToken>,
}

impl Recipe for ShutdownRecipe {
   type Output = Message;

   fn hash(&self, state: &mut rustc_hash::FxHasher) {
      std::any::TypeId::of::<Self>().hash(state);
   }

   fn stream(
      self: Box<Self>,
      _input: BoxStream<'static, iced::advanced::subscription::Event>,
   ) -> BoxStream<'static, Message> {
      let token = self.token.clone();
      Box::pin(async_stream::stream! {
         token.cancelled().await;
         yield Message::Shutdown;
      })
   }
}

fn namespace() -> String {
   String::from("dynamic_bar")
}

fn subscription(app: &App) -> iced::Subscription<Message> {
   iced::advanced::subscription::from_recipe(ShutdownRecipe {
      token: app.token.clone()
   })
}

fn update(app: &mut App, message: Message) -> Task<Message> {
   match message {
      Message::Shutdown => iced::exit(),
      _ => Task::none(),
   }
}

fn view(counter: &App) -> Element<Message> {
   iced::widget::column![]
      .width(iced::Length::Fill)
      .height(iced::Length::Fill)
      .into()
}

fn style(_counter: &App, theme: &iced::Theme) -> iced::theme::Style {
   use iced::theme::Style;
   Style {
      background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
      text_color: Color::TRANSPARENT,
   }
}
