// An enum representing the buttons that are universally available on gamepads.
#[derive(PartialEq, Debug)]
pub enum InputButton {
  North,
  South,
  East,
  West,
  LeftBumper,
  LeftTrigger,
  RightBumper,
  RightTrigger,
  Start,
  Select,
  DPadUp,
  DPadDown,
  DPadLeft,
  DPadRight
}

// An enum representing the axes that are universally available on gamepads.
#[derive(Debug)]
pub enum InputAxis {
  LeftX,
  LeftY,
  RightX,
  RightY
}

// An enum representing the different events possible on a gamepad.
#[derive(Debug)]
pub enum InputEvent {
  GamepadButton(usize, InputButton, f32),
  GamepadAxis(usize, InputAxis, f32)
}

impl InputEvent {
  // A method that returns that the gamepad ID of this event.
  pub fn get_gamepad_id(&self) -> &usize {
    return match self {
      Self::GamepadButton(gamepad_id, _, _) => gamepad_id,
      Self::GamepadAxis(gamepad_id, _, _) => gamepad_id
    }
  }
}

/**
 * A trait representing a input adapter that reads from an gamepad input library
 * of some kind, from which an input event can be generated.
 */
pub trait InputAdapter {
  /**
   * A method that reads from an input library's buffer and returns the buffered
   * events.
   */
  fn read(&mut self) -> Vec<InputEvent>;

  /**
   * A method that checks the input library to verify if a gamepad of a given ID
   * is connected.
   */
  fn is_connected(&mut self, gamepad_id: &usize) -> bool;
}
