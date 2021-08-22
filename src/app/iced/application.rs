use crate::{
  app::{
    common::ClientApp,
    iced::{
      style,
      widget::GamepadDisplay
    }
  },
  input::{
    adapter::sdl::SdlAdapter,
    switch::EmulatedPad
  },
  model::ClientModel,
  controller::ClientController,
  view::cli::CLIView
};

use iced::{
  button, Button,
  canvas::{self, Cache, Canvas, Cursor, Geometry},
  executor, Application, Clipboard, Column, Command, Container, Element, Length,
  Row,
  Subscription,
  Settings,
  Text
};
use std::collections::HashMap;

pub struct IcedApp {
  theme: style::Theme,
  controller: ClientController,
  started: bool,
  button_states: HashMap<String, button::State>,
  start_button_state: button::State,
  exit_button_state: button::State,
  status: String
}

impl IcedApp {
  pub fn new() -> IcedApp {
    match ClientModel::new() {
      Ok(model) => {
        return IcedApp {
          theme: style::Theme::Dark,
          controller: ClientController::new(model, Box::new(SdlAdapter::new())),
          button_states: HashMap::new(),
          start_button_state: button::State::new(),
          exit_button_state: button::State::new(),
          started: false,
          status: "".to_owned()
        }
      },
      Err(e) => panic!("{}", e)
    }
  }

  fn update_status(&mut self, status: Result<String, String>) -> () {
    match status {
      Ok(o) => self.status = format!("OK: {}", o),
      Err(e) => self.status = format!("ERR: {}", e)
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum ClientMessage {
  Tick,
  Start,
  Stop,
  Exit
}

impl ClientApp for IcedApp {
  fn initialize() -> () {
    Self::run(Settings {
      antialiasing: true,
      ..Settings::default()
    });
    ()
  }
}

impl Application for IcedApp {
  type Executor = executor::Default;
  type Message = ClientMessage;
  type Flags = ();

  fn new(_flags: ()) -> (Self, Command<Self::Message>) {
    let mut app: IcedApp = IcedApp::new();
    let result: Result<String, String> = app.controller.initialize();
    app.update_status(result);
    return (
      app,
      Command::none()
    )
  }

  fn title(&self) -> String {
    return "client-rs".to_owned();
  }

  fn update(
    &mut self, message: Self::Message, _clipboard: &mut Clipboard
  ) -> Command<Self::Message> {
    match message {
      Self::Message::Tick => {
        self.controller.update();
      },
      Self::Message::Start => {
        let result: Result<String, String> = self.controller.start();
        self.update_status(result);
        self.started = !self.started;
      },
      Self::Message::Stop => {
        let result: Result<String, String> = self.controller.stop();
        self.update_status(result);
        self.started = !self.started;
      },
      Self::Message::Exit => match self.controller.exit_prep() {
        Ok(_) => std::process::exit(0),
        Err(e) => panic!("{}", e)
      }
    }

    return Command::none();
  }

  fn subscription(&self) -> Subscription<Self::Message> {
    return iced::time::every(std::time::Duration::from_secs_f32(1.0 / 60.0))
      .map(|_| Self::Message::Tick)
  }

  fn view(&mut self) -> Element<Self::Message> {

    let mut column: Column<ClientMessage> = Column::new()
      .width(Length::Fill)
      .height(Length::Fill);

    let mut row: Row<ClientMessage> = Row::new()
      .width(Length::Fill)
      .height(Length::Fill);

    let mut sidebar: Column<Self::Message> = Column::new()
      .width(Length::FillPortion(1))
      .height(Length::Fill)
      .padding(10)
      .spacing(10);

    // let borrowed = &mut self.button_states;
    sidebar = sidebar.push(
      Button::new(
        &mut self.start_button_state,
        Text::new(
          match &self.started {
            true => "Stop",
            false => "Start"
          }
        )
      )
        .width(Length::Fill)
        .style(self.theme)
        .on_press(
          match &self.started {
            true => ClientMessage::Stop,
            false => ClientMessage::Start
          }
        )
    );

    sidebar = sidebar.push(
      Button::new(&mut self.exit_button_state, Text::new("Exit"))
        .width(Length::Fill)
        .style(self.theme)
        .on_press(ClientMessage::Exit)
    );

    row = row.push(sidebar);

    let mut main: Column<Self::Message> = Column::new()
      .width(Length::FillPortion(4))
      .height(Length::Fill)
      .padding(10);

    let pads: Vec<EmulatedPad> = self.controller.get_pads();
    let mut gamepad_rows: Column<ClientMessage> = Column::new()
      .spacing(10);
    let mut gamepad_row: Row<ClientMessage> = Row::new().spacing(10);
    for i in 0..4 {
      gamepad_row = gamepad_row.push(GamepadDisplay::new(self.theme, pads[i]));
    }
    let mut gamepad_row_2: Row<ClientMessage> = Row::new().spacing(10);
    for i in 4..8 {
      gamepad_row_2 = gamepad_row_2.push(GamepadDisplay::new(self.theme, pads[i]));
    }

    gamepad_rows = gamepad_rows
      .push(gamepad_row)
      .push(gamepad_row_2);

    main = main.push(gamepad_rows);

    row = row.push(main);

    column = column.push(row);

    let mut status_bar = Row::new()
      .width(Length::Fill)
      .height(Length::Shrink)
      .padding(10);

    status_bar = status_bar.push(Text::new(&self.status));

    column = column.push(status_bar);

    return Container::new(column)
      .width(Length::Fill)
      .height(Length::Fill)
      .style(self.theme)
      .into();
  }
}
