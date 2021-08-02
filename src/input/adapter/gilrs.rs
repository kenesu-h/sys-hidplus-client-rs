use crate::input::adapter::common::{
  InputButton,
  InputAxis,
  InputEvent,
  InputAdapter
};

use gilrs::{
  Gilrs,
  EventType,
  GamepadId,
  Button,
  Axis
};
use std::convert::TryInto;

/**
 * A struct representing a cross-platform input adapter that will read from a
 * GilRs instance.
 * 
 * As of the time of documentation, GilRs does not support any other gamepad
 * APIs on Windows other than XInput, and as a result will not support more than
 * 4 gamepads. This has only been tested on Windows as well, but should
 * theoretically work with Unix OS's.
 */
pub struct GilrsAdapter {
  gilrs: Gilrs
}

impl GilrsAdapter {
  // Constructs a GilRs input adapter with an accompanying GilRs instance.
  pub fn new() -> GilrsAdapter {
    return GilrsAdapter {
      gilrs: Gilrs::new().unwrap()
    }
  }

  // A helper method to adapt GilRs button event values into an InputEvent.
  fn to_button_event(
    &self, gamepad_id: &GamepadId, button: &Button, value: &f32
  ) -> Result<InputEvent, String> {
    return match self.to_button(button) {
      Ok(mapped) => Ok(
        InputEvent::GamepadButton(
          (*gamepad_id).try_into().unwrap(),
          mapped,
          *value
        )
      ),
      Err(e) => Err(e)
    }
  }

  // A helper method to adapt GilRs axis event values into an InputEvent.
  fn to_axis_event(
    &self, gamepad_id: &GamepadId, axis: &Axis, value: &f32
  ) -> Result<InputEvent, String> {
    return match self.to_axis(axis) {
      Ok(mapped) => Ok(
        InputEvent::GamepadAxis(
          (*gamepad_id).try_into().unwrap(),
          mapped,
          *value
        )
      ),
      Err(e) => Err(e)
    }
  }


  // A helper method to map GilRs buttons into InputButtons.
  fn to_button(&self, button: &Button) -> Result<InputButton, String> {
    return match button {
      Button::South => Ok(InputButton::South),
      Button::East => Ok(InputButton::East),
      Button::North => Ok(InputButton::North),
      Button::West => Ok(InputButton::West),
      Button::LeftTrigger => Ok(InputButton::LeftBumper),
      Button::LeftTrigger2 => Ok(InputButton::LeftTrigger),
      Button::RightTrigger => Ok(InputButton::RightBumper),
      Button::RightTrigger2 => Ok(InputButton::RightTrigger),
      Button::Start => Ok(InputButton::Start),
      Button::Select => Ok(InputButton::Select),
      Button::DPadUp => Ok(InputButton::DPadUp),
      Button::DPadDown => Ok(InputButton::DPadDown),
      Button::DPadLeft => Ok(InputButton::DPadLeft),
      Button::DPadRight => Ok(InputButton::DPadRight),
      _ => Err(format!("{:?} is currently an unmapped GilRs button.", button))
    }
  }

  // A helper method to map GilRs axes into InputAxes.
  fn to_axis(&self, axis: &Axis) -> Result<InputAxis, String> {
    return match axis {
      Axis::LeftStickX => Ok(InputAxis::LeftX),
      Axis::LeftStickY => Ok(InputAxis::LeftY),
      Axis::RightStickX => Ok(InputAxis::RightX),
      Axis::RightStickY => Ok(InputAxis::RightY),
      _ => Err(format!("{:?} is currently an unmapped GilRs axis.", axis))
    }
  }
}

impl InputAdapter for GilrsAdapter {
  fn read(&mut self) -> Vec<InputEvent> {
    let mut events: Vec<InputEvent> = vec!();
    while let Some(gilrs::Event { id: gamepad_id, event: event_type, time: _ })
      = self.gilrs.next_event() {
      match event_type {
        EventType::ButtonChanged(button, value, _) => {
          match self.to_button_event(&gamepad_id, &button, &value) {
            Ok(adapted) => events.push(adapted),
            Err(_) => ()
          }
        },
        EventType::AxisChanged(axis, value, _) => {
          match self.to_axis_event(&gamepad_id, &axis, &value) {
            Ok(adapted) => events.push(adapted),
            Err(_) => ()
          }
        },
        _ => ()
      }
    }
    return events;
  }

  /*
   * This could probably be more efficient if we could turn a usize into a
   * GamepadId, but we can't. Until then, this is gonna have to be O(n).
   */
  fn is_connected(&mut self, gamepad_id: &usize) -> bool {
    for (id, _) in self.gilrs.gamepads() {
      if *gamepad_id == id.try_into().unwrap() {
        return true;
      }
    }
    return false;
  }
}
