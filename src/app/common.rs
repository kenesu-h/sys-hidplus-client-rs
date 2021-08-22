pub trait ClientApp {
  fn initialize() -> ();
}

pub enum ClientMessage {
  Tick,
  Start,
  Stop,
  Restart,
  Exit,
  Help(String),
  SetServerIp(String),
  SetSwitchPad(String),
  SetInputDelay(String),
  SetLeftDeadzone(String),
  SetRightDeadzone(String)
}
