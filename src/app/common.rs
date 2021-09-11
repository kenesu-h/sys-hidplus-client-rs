use crate::input::switch::SwitchPad;

/**
 * Represents an application that is responsible for:
 * - Rendering the UI.
 * - Accepting user input and relaying it to the controller.
 */
pub trait ClientApp {
  // Initializes the application, starting its main loop.
  fn initialize() -> ();
}

/** 
 * Represents a variety of messages that can be broken down into methods of the
 * controller. Application structs are expected to parse user input into these
 * messages.
 */
#[derive(Debug, Clone)]
pub enum ClientMessage {
  Tick,
  Start,
  Stop,
  Restart,
  Exit,
  Help(Option<String>),
  SetServerIp(String),
  SetSwitchPad(usize, SwitchPad),
  SetInputDelay(usize, u8),
  SetLeftDeadzone(usize, f32),
  SetRightDeadzone(usize, f32),
  SetAnarchyMode(bool),

  // For the Iced GUI exclusively (and others that rely on similar messages)
  ServerIPInputChanged(String),
  InputDelayInputChanged(usize, String),
  LeftDeadzoneInputChanged(usize, f32),
  RightDeadzoneInputChanged(usize, f32),
  TrySetAll(usize, String, f32, f32),
  ScreenChanged(ClientScreen)
}

#[derive(Debug, Clone)]
pub enum ClientScreen {
  Main,
  GamepadConfig(usize)
}