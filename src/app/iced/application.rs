use crate::{
  app::{
    common::{ClientApp, ClientMessage, ClientScreen},
    iced::style
  },
  input::adapter::sdl::SdlAdapter,
  model::ClientModel,
  controller::ClientController
};

use iced::{
  Align,
  button, Button,
  tooltip::{self, Tooltip},
  executor, Application, Clipboard, Column, Command, Container, Element, Length,
  Row,
  slider, Slider,
  Subscription,
  Settings,
  Text,
  text_input, TextInput,
  window
};
use std::collections::HashSet;

pub enum IcedScreen {
  Main,
  GamepadConfig(usize)
}

pub struct IcedApp {
  theme: style::Theme,
  controller: ClientController,
  screen: ClientScreen,
  started: bool, 
  status: String,

  input_server_ip: text_input::State,
  value_server_ip: String,
  button_server_ip_save: button::State,

  button_gamepad_1: button::State,
  button_gamepad_2: button::State,
  button_gamepad_3: button::State,
  button_gamepad_4: button::State,
  button_gamepad_5: button::State,
  button_gamepad_6: button::State,
  button_gamepad_7: button::State,
  button_gamepad_8: button::State,

  button_manual_assign: button::State,
  button_anarchy: button::State,
  button_start: button::State,
  button_exit: button::State,

  input_input_delay: text_input::State,
  value_input_delay: String,
  slider_left_deadzone: slider::State,
  value_left_deadzone: u8,
  slider_right_deadzone: slider::State,
  value_right_deadzone: u8,
  button_gamepad_back: button::State,
  button_gamepad_save: button::State
}

fn format_gamepad_text(i: &usize, mapped: &HashSet<usize>) -> String {
    return format!(
      "{}: Gamepad {}",
      match mapped.contains(i) {
        true => "O",
        false => "X"
      },
      i + 1
    )
  }

impl IcedApp {
  pub fn new() -> IcedApp {
    match ClientModel::new() {
      Ok(model) => {
        return IcedApp {
          theme: style::Theme::Dark,
          controller: ClientController::new(model, Box::new(SdlAdapter::new())),
          screen: ClientScreen::Main,
          started: false,
          status: "".to_owned(),

          input_server_ip: text_input::State::new(),
          value_server_ip: String::new(),
          button_server_ip_save: button::State::new(),

          button_gamepad_1: button::State::new(),
          button_gamepad_2: button::State::new(),
          button_gamepad_3: button::State::new(),
          button_gamepad_4: button::State::new(),
          button_gamepad_5: button::State::new(),
          button_gamepad_6: button::State::new(),
          button_gamepad_7: button::State::new(),
          button_gamepad_8: button::State::new(),

          button_manual_assign: button::State::new(),
          button_anarchy: button::State::new(),
          button_start: button::State::new(),
          button_exit: button::State::new(), 

          input_input_delay: text_input::State::new(),
          value_input_delay: String::new(),
          slider_left_deadzone: slider::State::new(),
          value_left_deadzone: 0,
          slider_right_deadzone: slider::State::new(),
          value_right_deadzone: 0,

          button_gamepad_back: button::State::new(),
          button_gamepad_save: button::State::new()
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

  

  fn view_main(&mut self) -> Element<ClientMessage> {
    let mut rows: Column<ClientMessage> = Column::new()
      .width(Length::Fill)
      .height(Length::Fill)
      .padding(10)
      .spacing(20);

    let mut buttons: Row<ClientMessage> = Row::new()
      .spacing(10);

    let mut server_ip_row: Row<ClientMessage> = Row::new()
      .spacing(10)
      .align_items(Align::Center);

    server_ip_row = server_ip_row.push(Text::new("Server IP"));

    server_ip_row = server_ip_row.push(
      TextInput::new(
        &mut self.input_server_ip,
        "Type your Nintendo Switch's IP here, then save.",
        &self.value_server_ip,
        ClientMessage::ServerIPInputChanged
      )
        .width(Length::Fill)
        .padding(5)
        .style(self.theme)
    );

    server_ip_row = server_ip_row.push(
      Button::new(
        &mut self.button_server_ip_save,
        Text::new("Save"),
      )
        .style(self.theme)
        .width(Length::Shrink)
        .on_press(ClientMessage::SetServerIp(self.value_server_ip.clone()))
    );

    rows = rows.push(server_ip_row);

    let mut gamepad_row: Column<ClientMessage> = Column::new()
      .spacing(10);

    gamepad_row = gamepad_row.push(Text::new("Gamepads").size(24));
    
    let mut gamepad_row_1: Row<ClientMessage> = Row::new()
      .spacing(10);

    let mapped: HashSet<usize> = self.controller.get_input_map().values().cloned().collect();

    /* I really hate to be repetitious here but the borrow checker gives me no
     * other options.
     */
    gamepad_row_1 = gamepad_row_1.push(
      Button::new(
        &mut self.button_gamepad_1,
        Text::new(format_gamepad_text(&0, &mapped))
      )
        .style(self.theme)
        .width(Length::Shrink)
        .on_press(ClientMessage::ScreenChanged(ClientScreen::GamepadConfig(0)))
    );

    gamepad_row_1 = gamepad_row_1.push(
      Button::new(
        &mut self.button_gamepad_2,
        Text::new(format_gamepad_text(&1, &mapped))
      )
        .style(self.theme)
        .width(Length::Shrink)
        .on_press(ClientMessage::ScreenChanged(ClientScreen::GamepadConfig(1)))
    );
    
    gamepad_row_1 = gamepad_row_1.push(
      Button::new(
        &mut self.button_gamepad_3,
        Text::new(format_gamepad_text(&2, &mapped))
      )
        .style(self.theme)
        .width(Length::Shrink)
        .on_press(ClientMessage::ScreenChanged(ClientScreen::GamepadConfig(2)))
    );
    
    gamepad_row_1 = gamepad_row_1.push(
      Button::new(
        &mut self.button_gamepad_4,
        Text::new(format_gamepad_text(&3, &mapped))
      )
        .style(self.theme)
        .width(Length::Shrink)
        .on_press(ClientMessage::ScreenChanged(ClientScreen::GamepadConfig(3)))
    );

    let mut gamepad_row_2: Row<ClientMessage> = Row::new()
      .spacing(10);

    gamepad_row_2 = gamepad_row_2.push(
      Button::new(
        &mut self.button_gamepad_5,
        Text::new(format_gamepad_text(&4, &mapped))
      )
        .style(self.theme)
        .width(Length::Shrink)
        .on_press(ClientMessage::ScreenChanged(ClientScreen::GamepadConfig(4)))
    );

    gamepad_row_2 = gamepad_row_2.push(
      Button::new(
        &mut self.button_gamepad_6,
        Text::new(format_gamepad_text(&5, &mapped))
      )
        .style(self.theme)
        .width(Length::Shrink)
        .on_press(ClientMessage::ScreenChanged(ClientScreen::GamepadConfig(5)))
    );
    
    gamepad_row_2 = gamepad_row_2.push(
      Button::new(
        &mut self.button_gamepad_7,
        Text::new(format_gamepad_text(&6, &mapped))
      )
        .style(self.theme)
        .width(Length::Shrink)
        .on_press(ClientMessage::ScreenChanged(ClientScreen::GamepadConfig(6)))
    );
    
    gamepad_row_2 = gamepad_row_2.push(
      Button::new(
        &mut self.button_gamepad_8,
        Text::new(format_gamepad_text(&7, &mapped))
      )
        .style(self.theme)
        .width(Length::Shrink)
        .on_press(ClientMessage::ScreenChanged(ClientScreen::GamepadConfig(7)))
    );

    gamepad_row = gamepad_row.push(gamepad_row_1);
    gamepad_row = gamepad_row.push(gamepad_row_2);

    rows = rows.push(
      Container::new(gamepad_row)
        .width(Length::Fill)
        .height(Length::Shrink)
        .center_x()
    );

    buttons = buttons.push(
      Tooltip::new(
        Button::new(
          &mut self.button_manual_assign,
          Text::new(
            match &self.controller.get_manual_assign() {
              true => "Manual Assign On",
              false => "Manual Assign Off"
            }
          )
        )
          .width(Length::Shrink)
          .style(self.theme)
          .on_press(
            match &self.controller.get_manual_assign() {
              true => ClientMessage::SetManualAssign(false),
              false => ClientMessage::SetManualAssign(true)
            }
          ),
        "Toggles whether right bumper must be pressed to assign controllers.",
        tooltip::Position::Bottom
      )
    );

    buttons = buttons.push(
      Tooltip::new(
        Button::new(
          &mut self.button_anarchy,
          Text::new(
            match &self.controller.get_anarchy_mode() {
              true => "Anarchy Mode On",
              false => "Anarchy Mode Off"
            }
          )
        )
          .width(Length::Shrink)
          .style(self.theme)
          .on_press(
            match &self.controller.get_anarchy_mode() {
              true => ClientMessage::SetAnarchyMode(false),
              false => ClientMessage::SetAnarchyMode(true)
            }
          ),
        "Everyone controls one gamepad. Disconnects all gamepads on toggle.",
        tooltip::Position::Bottom
      )
    );

    buttons = buttons.push(
      Tooltip::new(
        Button::new(
          &mut self.button_start,
          Text::new(
            match &self.started {
              true => "Stop Client",
              false => "Start Client"
            }
          )
        )
          .width(Length::Shrink)
          .style(self.theme)
          .on_press(
            match &self.controller.get_running() {
              true => ClientMessage::Stop,
              false => ClientMessage::Start
            }
          ),
        match &self.controller.get_running() {
          true => "Stops the client and disconnects from the Nintendo Switch.",
          false => "Starts the client and connects to the Nintendo Switch."
        },
        tooltip::Position::Bottom
      )
    );

    buttons = buttons.push(
      Tooltip::new(
        Button::new(&mut self.button_exit, Text::new("Exit"))
          .width(Length::Shrink)
          .style(self.theme)
          .on_press(ClientMessage::Exit),
        "Exits the client, disconnecting from the Nintendo Switch if needed.",
        tooltip::Position::Bottom
      )
    );

    rows = rows.push(
      Container::new(buttons)
        .width(Length::Fill)
        .height(Length::Shrink)
        .center_x()
    );

    let mut status_bar = Row::new()
      .width(Length::Fill)
      .height(Length::Shrink)
      .padding(10);

    status_bar = status_bar.push(Text::new(&self.status));

    rows = rows.push(status_bar);

    return Container::new(rows)
      .width(Length::Fill)
      .height(Length::Fill)
      .style(self.theme)
      .into();
  } 

  fn view_gamepad_config(&mut self, i: &usize) -> Element<ClientMessage> {
    let mut rows: Column<ClientMessage> = Column::new()
      .width(Length::Fill)
      .height(Length::Fill)
      .padding(10)
      .spacing(20);

    rows = rows.push(
      Text::new(format!("Gamepad {} Configuration", i + 1)).size(24)
    );

    let mut input_delay_row: Row<ClientMessage> = Row::new()
      .spacing(10)
      .align_items(Align::Center);

    input_delay_row = input_delay_row.push(
      Text::new("Input Delay")
        .width(Length::FillPortion(1))
    );

    input_delay_row = input_delay_row.push(
      TextInput::new(
        &mut self.input_input_delay,
        "Value must be in range [0 to 255].",
        &self.value_input_delay,
        move |maybe_input_delay| {
          ClientMessage::InputDelayInputChanged(maybe_input_delay)
        }
      )
        .style(self.theme)
        .width(Length::FillPortion(1))
        .padding(5)
    );

    rows = rows.push(input_delay_row);

    let mut left_deadzone_row: Row<ClientMessage> = Row::new()
      .spacing(10)
      .align_items(Align::Center);

    left_deadzone_row = left_deadzone_row.push(
      Text::new("Left Deadzone")
        .width(Length::FillPortion(1))
    );
    
    left_deadzone_row = left_deadzone_row.push(
      Text::new(format!("{}%", self.value_left_deadzone))
        .width(Length::FillPortion(1))
    );

    left_deadzone_row = left_deadzone_row.push(
      Slider::new(
        &mut self.slider_left_deadzone,
        0..=100,
        self.value_left_deadzone,
        move |deadzone| {
          ClientMessage::LeftDeadzoneInputChanged(deadzone as f32 / 100.0)
        }
      )
        .style(self.theme)
        .width(Length::FillPortion(2))
      /*
      TextInput::new(
        &mut self.input_left_deadzone,
        "Value must be a decimal greater than 0.0.",
        &self.value_left_deadzone,
        move |maybe_deadzone| {
          ClientMessage::LeftDeadzoneInputChanged(i_clone, maybe_deadzone)
        }
      )
        .style(self.theme)
        .width(Length::Fill)
        .padding(5)
      */
    );

    rows = rows.push(left_deadzone_row);

    let mut right_deadzone_row: Row<ClientMessage> = Row::new()
      .spacing(10)
      .align_items(Align::Center);

    right_deadzone_row = right_deadzone_row.push(
      Text::new("Right Deadzone")
        .width(Length::FillPortion(1))
    );
    
    right_deadzone_row = right_deadzone_row.push(
      Text::new(format!("{}%", self.value_right_deadzone))
        .width(Length::FillPortion(1))
    );

    right_deadzone_row = right_deadzone_row.push(
      Slider::new(
        &mut self.slider_right_deadzone,
        0..=100,
        self.value_right_deadzone,
        move |deadzone| {
          ClientMessage::RightDeadzoneInputChanged(deadzone as f32 / 100.0 )
        }
      )
        .style(self.theme)
        .width(Length::FillPortion(2))
      /*
      TextInput::new(
        &mut self.input_right_deadzone,
        "Value must be a decimal greater than 0.0.",
        &self.value_left_deadzone,
        move |maybe_deadzone| {
          ClientMessage::RightDeadzoneInputChanged(i_clone, maybe_deadzone)
        }
      )
        .style(self.theme)
        .width(Length::Fill)
        .padding(5)
      */
    );

    rows = rows.push(right_deadzone_row);

    let mut buttons: Row<ClientMessage> = Row::new()
      .spacing(10);

    buttons = buttons.push(
      Button::new(&mut self.button_gamepad_back, Text::new("Back"))
        .width(Length::Shrink)
        .style(self.theme)
        .on_press(ClientMessage::ScreenChanged(ClientScreen::Main))
    );

    buttons = buttons.push(
      Button::new(&mut self.button_gamepad_save, Text::new("Save"))
        .style(self.theme)
        .width(Length::Shrink)
        .on_press(
          ClientMessage::TrySetAll(
            *i,
            self.value_input_delay.clone(),
            self.value_left_deadzone.clone() as f32 / 100.0,
            self.value_right_deadzone.clone() as f32 / 100.0
          )
        )
    );

    rows = rows.push(buttons);

    let mut status_bar = Row::new()
      .width(Length::Fill)
      .height(Length::Shrink)
      .padding(10);

    status_bar = status_bar.push(Text::new(&self.status));

    rows = rows.push(status_bar);

    return Container::new(rows)
      .width(Length::Fill)
      .height(Length::Fill)
      .style(self.theme)
      .into();
  }
}

impl ClientApp for IcedApp {
  fn initialize() -> () {
    match Self::run(Settings {
      antialiasing: true,
      window: window::Settings {
        size: (640, 480),
        ..window::Settings::default()
      },
      ..Settings::default()
    }) {
      Ok(_) => (),
      Err(e) => panic!(
        "Initializing the client resulted in the following error: {}",
        e
      )
    }
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
    app.value_server_ip = app.controller.get_server_ip().clone();
    return (
      app,
      Command::none()
    )
  }

  fn title(&self) -> String {
    return "client-rs".to_owned();
  }

  fn update(
    &mut self, message: ClientMessage, _clipboard: &mut Clipboard
  ) -> Command<ClientMessage> {
    match message {
      ClientMessage::Tick => {
        self.controller.update();
      },
      ClientMessage::Start => {
        let result: Result<String, String> = self.controller.start();
        self.update_status(result);
        self.started = !self.started;
      },
      ClientMessage::Stop => {
        let result: Result<String, String> = self.controller.stop();
        self.update_status(result);
        self.started = !self.started;
      },
      ClientMessage::Restart => (),
      ClientMessage::Exit => match self.controller.exit_prep() {
        Ok(_) => std::process::exit(0),
        Err(e) => panic!("{}", e)
      },
      ClientMessage::Help(_) => (),
      ClientMessage::SetServerIp(server_ip) => {
        let result: Result<String, String> =
          self.controller.set_server_ip(&server_ip);
        self.update_status(result); 
      },
      ClientMessage::SetSwitchPad(i, switch_pad) => {
        let result: Result<String, String> =
          self.controller.set_switch_pad(&i, &switch_pad);
        self.update_status(result);
      },
      ClientMessage::SetInputDelay(i, input_delay) => {
        let result: Result<String, String> =
          self.controller.set_input_delay(&i, &input_delay);
        self.update_status(result);
      },
      ClientMessage::SetLeftDeadzone(i, deadzone) => {
        let result: Result<String, String> =
          self.controller.set_left_deadzone(&i, &deadzone);
        self.update_status(result);
      },
      ClientMessage::SetRightDeadzone(i, deadzone) => {
        let result: Result<String, String> =
          self.controller.set_right_deadzone(&i, &deadzone);
        self.update_status(result);
      },
      ClientMessage::TrySetAll(i, maybe_input_delay, left_deadzone, right_deadzone) => {
        let mut failed: Vec<String> = vec!();

        match maybe_input_delay.parse::<u8>() {
          Ok(input_delay) => {
            match self.controller.set_input_delay(&i, &input_delay) {
              Ok(_) => (),
              Err(_) => failed.push(String::from("Input Delay"))
            }
          },
          Err(_) => failed.push(String::from("Input Delay"))
        }

        match self.controller.set_left_deadzone(&i, &left_deadzone) {
          Ok(_) => (),
          Err(_) => failed.push(String::from("Left Deadzone"))
        }

        match self.controller.set_right_deadzone(&i, &right_deadzone) {
          Ok(_) => (),
          Err(_) => failed.push(String::from("Right Deadzone"))
        }

        if failed.len() == 0 {
          self.update_status(Ok(String::from("Config successfully saved.")));
        } else {
          self.update_status(
            Err(
              format!("The following fields failed to save: {:?}", failed)
            )
          )
        }
      },
      ClientMessage::SetManualAssign(manual_assign) => {
        let result: Result<String, String> =
          self.controller.set_manual_assign(&manual_assign);
        self.update_status(result);
      }
      ClientMessage::SetAnarchyMode(anarchy_mode) => {
        let result: Result<String, String> =
          self.controller.set_anarchy_mode(&anarchy_mode);
        self.update_status(result);
      },
      ClientMessage::ServerIPInputChanged(server_ip) => {
        self.value_server_ip = server_ip;
      },
      ClientMessage::InputDelayInputChanged(maybe_input_delay) => {
        self.value_input_delay = maybe_input_delay;
      },
      ClientMessage::LeftDeadzoneInputChanged(deadzone) => {
        self.value_left_deadzone = (deadzone * 100.0) as u8;
      },
      ClientMessage::RightDeadzoneInputChanged(deadzone) => {
        self.value_right_deadzone = (deadzone * 100.0) as u8;
      },
      ClientMessage::ScreenChanged(screen) => {
        self.screen = screen.clone();
        match screen {
          ClientScreen::GamepadConfig(i) => {
            self.value_input_delay =
              self.controller.get_input_delay(&i).to_string();
            self.value_left_deadzone = (*self.controller.get_left_deadzone(&i) * 100.0) as u8;
            self.value_right_deadzone = (*self.controller.get_right_deadzone(&i) * 100.0) as u8;
          },
          _ => ()
        }
      }
    }

    return Command::none();
  }

  fn subscription(&self) -> Subscription<ClientMessage> {
    return iced::time::every(std::time::Duration::from_secs_f32(1.0 / 60.0))
      .map(|_| Self::Message::Tick)
  }

  fn view(&mut self) -> Element<ClientMessage> {
    return match self.screen {
      ClientScreen::Main => self.view_main(),
      ClientScreen::GamepadConfig(i) => self.view_gamepad_config(&i)
    }
  }
}