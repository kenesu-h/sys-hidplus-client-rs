// Represents the buttons that are universally available on gamepads.
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

// Represents the axes that are universally available on gamepads.
#[derive(Debug)]
pub enum InputAxis {
  LeftX,
  LeftY,
  RightX,
  RightY
}

// Represents the different events possible on a gamepad.
#[derive(Debug)]
pub enum InputEvent {
  GamepadButton(usize, InputButton, f32),
  GamepadAxis(usize, InputAxis, f32)
}

impl InputEvent {
  // Returns that the gamepad ID of this event.
  pub fn get_gamepad_id(&self) -> &usize {
    return match self {
      Self::GamepadButton(gamepad_id, _, _) => gamepad_id,
      Self::GamepadAxis(gamepad_id, _, _) => gamepad_id
    }
  }
}

/**
 * Represents a input adapter that reads from an gamepad input library of some
 * kind, from which an input event can be generated.
 */
pub trait InputAdapter {
  // Reads from an input library's buffer and returns the buffered events.
  fn read(&mut self) -> Vec<InputEvent>;

  // Checks the input library to verify if a gamepad of a given ID is connected.
  fn is_connected(&mut self, gamepad_id: &usize) -> bool;
}
