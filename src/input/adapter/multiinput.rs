use crate::input::adapter::common::{
  InputButton,
  InputAxis,
  InputEvent,
  InputAdapter
};

use multiinput::{
  RawInputManager,
  DeviceType,
  Axis,
  HatSwitch,
  State,
  RawEvent, 
  XInputInclude
};

/**
 * A struct representing a RawInput input adapter that will read from the
 * multiinput library using an instance of an input manager.
 * 
 * This input reader is ONLY meant to be used for RawInput devices, and at the
 * time of writing this, has only been tested with DS4s (PS4 controllers).
 * XInput support is poor right now and gamepads other than the DS4 have not
 * been tested. Do not expect an exquisite amount of support from this.
 */
pub struct MultiInputAdapter {
  manager: RawInputManager
}

impl MultiInputAdapter {
  /**
   * Constructs a multiinput adapter with an input manager instance.
   * 
   * This input manager instance will not read from XInput devices or mouse &
   * keyboard, although the options exist and may be implemented in a later
   * update.
   */
  pub fn new() -> MultiInputAdapter {
    let mut manager: RawInputManager = RawInputManager::new().unwrap();
    manager.register_devices(
      DeviceType::Joysticks(
        /*
         * This was initially true, but XInput controller support was poor and
         * there was no way to return the type of a controller.
         */
        XInputInclude::False
      )
    );
    return MultiInputAdapter {
      manager: manager
    }
  } 

  // A helper method to parse a list of buffered events into InputEvents.
  fn parse_buffered(&mut self, buffered: Vec<RawEvent>) -> Vec<InputEvent> {
    let mut events: Vec<InputEvent> = vec!();
    for event in buffered {
      match event {
        RawEvent::JoystickButtonEvent(device_id, button, state) => {
          match self.to_button_event(&device_id, &button, &state) {
            Ok(adapted) => events.push(adapted),
            Err(_) => ()
          }
        },
        RawEvent::JoystickAxisEvent(device_id, axis, value) => {
          match self.to_axis_event(&device_id, &axis, &value) {
            Ok(adapted) => events.push(adapted),
            Err(_) => ()
          }
        },
        RawEvent::JoystickHatSwitchEvent(device_id, hat_switch) => {
          let dpad_input: MultiInputDPad = self.to_dpad(&hat_switch);
          for (button, value) in dpad_input.to_vec() {
            events.push(
              InputEvent::GamepadButton(
                device_id,
                button,
                value
              )
            )
          }
        }
        _ => ()
      }
    }
    return events;
  }

  // A helper method to adapt multiinput button event values into an InputEvent.
  fn to_button_event(
    &self, device_id: &usize, button: &usize, state: &State
  ) -> Result<InputEvent, String> {
    return match self.to_button(button) {
      Ok(mapped) => Ok(
        InputEvent::GamepadButton(
          *device_id,
          mapped,
          self.to_button_value(state)
        )
      ),
      Err(e) => Err(e)
    }
  }

  // A helper method to adapt multiinput axis event values into an InputEvent.
  fn to_axis_event(
    &self, device_id: &usize, axis: &Axis, value: &f64
  ) -> Result<InputEvent, String> {
    return match self.to_axis(axis) {
      Ok(mapped) => Ok(
        InputEvent::GamepadAxis(
          *device_id,
          mapped,
          self.correct_axis_value(axis, value)
        )
      ),
      Err(e) => Err(e)
    }
  }

  /**
   * A method that "corrects" a value for an axis, assuming the gamepad involved
   * is a DS4.
   *
   * For some reason, the right stick uses the Z and RZ axes; Z for horizontal
   * and RZ for vertical. Their values also happen to be inverted, unlike the
   * left stick. We use this method to invert the value back if it happens to be
   * Z or RZ.
   */
  fn correct_axis_value(&self, axis: &Axis, value: &f64) -> f32 {
    return match axis {
      Axis::Z | Axis::RZ => -(*value as f32),
      _ => *value as f32
    }
  }

  // A helper method to map a button (in the form of a usize) to an InputButton.
  fn to_button(&self, button: &usize) -> Result<InputButton, String> {
    return match button {
      0 => Ok(InputButton::West),
      1 => Ok(InputButton::South),
      2 => Ok(InputButton::East),
      3 => Ok(InputButton::North),
      4 => Ok(InputButton::LeftBumper),
      5 => Ok(InputButton::RightBumper),
      6 => Ok(InputButton::LeftTrigger),
      7 => Ok(InputButton::RightTrigger),
      8 => Ok(InputButton::Select),
      9 => Ok(InputButton::Start),
      _ => Err(format!("{:?} is currently an unmapped multiinput button.", button))
    }
  }

  // A helper method to map a button state to a value.
  fn to_button_value(&self, state: &State) -> f32 {
    return match state {
      State::Pressed => 1.0,
      State::Released => 0.0
    }
  }

  // A helper method to map an axis to an InputAxis.
  fn to_axis(&self, axis: &Axis) -> Result<InputAxis, String> {
    return match axis {
      Axis::X => Ok(InputAxis::LeftX),
      Axis::Y => Ok(InputAxis::LeftY),
      Axis::Z => Ok(InputAxis::RightX),
      Axis::RZ => Ok(InputAxis::RightY),
      _ => Err(format!("{:?} is currently an unmapped multiinput axis.", axis))
    }
  }

  // A helper method to adapt a hat switch to a MultiInputDPad.
  fn to_dpad(&self, hat_switch: &multiinput::HatSwitch) -> MultiInputDPad {
    return match hat_switch {
      HatSwitch::Center => MultiInputDPad::new(false, false, false, false),
      HatSwitch::Up => MultiInputDPad::new(true, false, false, false),
      HatSwitch::UpRight => MultiInputDPad::new(true, false, false, true),
      HatSwitch::Right => MultiInputDPad::new(false, false, false, true),
      HatSwitch::DownRight => MultiInputDPad::new(false, true, false, true),
      HatSwitch::Down => MultiInputDPad::new(false, true, false, false),
      HatSwitch::DownLeft => MultiInputDPad::new(false, true, true, false),
      HatSwitch::Left => MultiInputDPad::new(false, false, true, false),
      HatSwitch::UpLeft => MultiInputDPad::new(true, false, true, false)
    }
  }
}

impl InputAdapter for MultiInputAdapter {
  fn read(&mut self) -> Vec<InputEvent> {
    let mut buffered: Vec<RawEvent> = vec!();
    while let Some(event) = self.manager.get_event() {
      buffered.push(event); 
    }
    return self.parse_buffered(buffered);
  }

  fn is_connected(&mut self, gamepad_id: &usize) -> bool {
    return self.manager.get_joystick_state(*gamepad_id).is_some();
  }
}

/**
 * A struct intended to help represent d-pad inputs in a more intuitive way as opposed to a list of
 * InputEvents.
 *
 * Each field represents whether that cardinal direction is pressed.
 */
struct MultiInputDPad {
  up: bool,
  down: bool,
  left: bool,
  right: bool
}

impl MultiInputDPad {
  // Constructs a new MultiInputDPad from whether the given directions are pressed.
  pub fn new(up: bool, down: bool, left: bool, right: bool) -> MultiInputDPad {
    return MultiInputDPad {
      up: up,
      down: down,
      left: left,
      right: right
    }
  }

  // A method to convert the directions into a list of pairs of (InputButton, value as f32).
  pub fn to_vec(&self) -> Vec<(InputButton, f32)> {
    return vec!(
      (InputButton::DPadUp, self.bool_to_value(&self.up)),
      (InputButton::DPadDown, self.bool_to_value(&self.down)),
      (InputButton::DPadLeft, self.bool_to_value(&self.left)),
      (InputButton::DPadRight, self.bool_to_value(&self.right))
    )
  }

  // A helper method to convert bools into their respective values.
  fn bool_to_value(&self, b: &bool) -> f32 {
    return match b {
      true => 1.0,
      false => 0.0
    }
  }
}
