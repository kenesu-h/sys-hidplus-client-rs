use crate::input::switch::SwitchPad;

pub trait ClientApp {
  fn initialize() -> ();
}

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
  SetRightDeadzone(usize, f32)
}
