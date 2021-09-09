extern crate sdl2;

use crate::input::adapter::common::{
  InputButton,
  InputAxis,
  InputEvent,
  InputAdapter
};

use sdl2::{
  Sdl,
  GameControllerSubsystem,
  JoystickSubsystem,
  EventPump,
  event::Event,
  controller::{
    GameController,
    Button,
    Axis
  },
  VideoSubsystem
};
use std::{
  collections::HashMap,
  panic,
  panic::AssertUnwindSafe
};

/**
 * Represents a cross-platform input adapter that will read from an SDL
 * instance.
 * 
 * SDL so far seems to bypass the 4 XInput controller limit and supports both
 * Xbox and PS4 controllers. Not sure how it performs cross-platform-wise, and
 * external drivers are still needed for controllers like GameCube controllers,
 * but this is revolutionary.
 *
 * _joystick and _video must be initialized in order for Xbox and controller
 * hotplugging to be supported. I really don't know why this is.
 */
pub struct SdlAdapter {
  gamepads: HashMap<u32, GameController>,
  game_controller: GameControllerSubsystem, 
  event_pump: EventPump,
  _joystick: JoystickSubsystem,
  _video: VideoSubsystem
}

impl SdlAdapter {
  pub fn new() -> SdlAdapter {
    let sdl_context: Sdl = sdl2::init().unwrap();

    let game_controller: GameControllerSubsystem =
      sdl_context.game_controller().unwrap();
    let joystick: JoystickSubsystem = sdl_context.joystick().unwrap();
    let event_pump: EventPump = sdl_context.event_pump().unwrap();
    let video: VideoSubsystem = sdl_context.video().unwrap();

    return SdlAdapter {
      gamepads: HashMap::new(),
      game_controller: game_controller, 
      event_pump: event_pump,
      _joystick: joystick,
      _video: video
    }
  }

  // Converts the components of an SDL button event to an InputEvent.
  fn to_button_event(
    &self, which: &u32, button: &Button, pressed: bool
  ) -> Result<InputEvent, String> {
    return match self.to_button(which, button) {
      Ok(mapped) => Ok(
        InputEvent::GamepadButton(
          *which as usize,
          mapped,
          self.to_button_value(pressed)
        )
      ),
      Err(e) => Err(e)
    }
  }

  // Maps whether a button is pressed to its respective f32 value.
  fn to_button_value(&self, pressed: bool) -> i16 {
    return match pressed {
      true => 1,
      false => 0
    }
  }

  // Maps an SDL button to an InputButton.
  fn to_button(&self, which: &u32, button: &Button) -> Result<InputButton, String> {
    return match button {
      /* TODO: I know it's just a band-aid solution to wrong mappings, but this
       * could probably be abstracted somehow.
       */
      Button::A => match self.gamepads.get(which) {
        Some(gamepad) => match gamepad.name().as_str() {
          "Nintendo Switch Pro Controller" => Ok(InputButton::East),
          _ => Ok(InputButton::South)
        },
        None => Err(String::from("Couldn't get controller name from index."))
      },
      Button::B => match self.gamepads.get(which) {
        Some(gamepad) => match gamepad.name().as_str() {
          "Nintendo Switch Pro Controller" => Ok(InputButton::South),
          _ => Ok(InputButton::East)
        },
        None => Err(String::from("Couldn't get controller name from index."))
      },
      Button::X => match self.gamepads.get(which) {
        Some(gamepad) => match gamepad.name().as_str() {
          "Nintendo Switch Pro Controller" => Ok(InputButton::North),
          _ => Ok(InputButton::West)
        },
        None => Err(String::from("Couldn't get controller name from index."))
      },
      Button::Y => match self.gamepads.get(which) {
        Some(gamepad) => match gamepad.name().as_str() {
          "Nintendo Switch Pro Controller" => Ok(InputButton::West),
          _ => Ok(InputButton::North)
        },
        None => Err(String::from("Couldn't get controller name from index."))
      },
      Button::Back => Ok(InputButton::Select),
      Button::Guide => Ok(InputButton::Guide),
      Button::Start => Ok(InputButton::Start),
      Button::LeftStick => Ok(InputButton::LeftStick),
      Button::RightStick => Ok(InputButton::RightStick),
      Button::LeftShoulder => Ok(InputButton::LeftBumper),
      Button::RightShoulder => Ok(InputButton::RightBumper),
      Button::DPadUp => Ok(InputButton::DPadUp),
      Button::DPadDown => Ok(InputButton::DPadDown),
      Button::DPadLeft => Ok(InputButton::DPadLeft),
      Button::DPadRight => Ok(InputButton::DPadRight)
    }
  }

  // Converts the components of an SDL axis event to an InputEvent.
  fn to_axis_event(
    &self, which: &u32, axis: &Axis, value: &i16
  ) -> Result<InputEvent, String> {
    return match self.to_axis(axis) {
      Ok(mapped) => Ok(
        InputEvent::GamepadAxis(
          *which as usize,
          mapped,
          self.to_axis_value(axis, value)
        )
      ),
      Err(e) => Err(e)
    }
  }

  // Maps an SDL axis to an InputAxis.
  fn to_axis(&self, axis: &Axis) -> Result<InputAxis, String> {
    return match axis {
      Axis::LeftX => Ok(InputAxis::LeftX),
      Axis::LeftY => Ok(InputAxis::LeftY),
      Axis::RightX => Ok(InputAxis::RightX),
      Axis::RightY => Ok(InputAxis::RightY),
      _ => Err(
        format!("{:?} is currently an unmapped SDL axis.", axis)
      )
    }
  } 

  /**
   * Adjusts the integer value of an SDL axis event.
   *
   * SDL inverts the Y-axes for the left and right analog sticks, so we need
   * to invert them to make them more intuitive (up should be positive, down
   * should be negative). However, since their values are 16-bit integers,
   * they are within a range of [-32768, 32767]. This means that we'd be
   * turning -32768 into 32768 upon inversion, which is a problem since it's
   * outside the range of an i16 (and will crash the client). -32768 is
   * instead mapped to 32767 as a result.
   */
  fn to_axis_value(&self, axis: &Axis, value: &i16) -> i16 {
    return match axis {
      Axis::LeftY | Axis::RightY => i16::saturating_mul(*value, -1),
      _ => *value
    }
  } 

  // Converts the components of an SDL trigger event to an InputEvent.
  fn to_trigger_event(
    &self, which: &u32, axis: &Axis, value: &i16
  ) -> Result<InputEvent, String> {
    return match self.to_trigger(axis) {
      Ok(mapped) => Ok(
        InputEvent::GamepadButton(
          *which as usize,
          mapped,
          /*
           * For a trigger to count as pressed, we're requiring this must be
           * pressed all the way. This MIGHT have potential issues later on.
           */
          self.to_button_value(value == &32767)
        )
      ),
      Err(e) => Err(e)
    }
  } 

  // Maps an SDL axis to an InputButton representing a trigger.
  fn to_trigger(&self, axis: &Axis) -> Result<InputButton, String> {
    return match axis {
      Axis::TriggerLeft => Ok(InputButton::LeftTrigger),
      Axis::TriggerRight => Ok(InputButton::RightTrigger),
      _ => Err(
        format!("{:?} is not a trigger axis.", axis)
      )
    }
  }

  // Returns whether the given SDL axis is a trigger.
  fn is_trigger(&self, axis: &Axis) -> bool {
    return match axis {
      Axis::TriggerLeft | Axis::TriggerRight => true,
      _ => false
    }
  }
}

impl InputAdapter for SdlAdapter {
  fn read(&mut self) -> Vec<InputEvent> {
    let mut events: Vec<InputEvent> = vec!();

    /* Disable the panic hook so we ignore panics from pressing an unmapped
     * button. We can't do much else until rust-sdl2 fixes this.
     *
     * rust-sdl2 issue: https://github.com/Rust-SDL2/rust-sdl2/issues/1128
     */
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    loop {
      match panic::catch_unwind(
        AssertUnwindSafe(|| { self.event_pump.poll_event() })
      ) {
        Ok(polled) => match polled {
          Some(event) =>  match event { 
            Event::ControllerDeviceAdded { which, .. } => {
              // We need to store the gamepad somewhere to receive button events.
              let gamepad: GameController = self.game_controller.open(which)
                .unwrap();
              self.gamepads.insert(gamepad.instance_id(), gamepad); 
            },
            Event::ControllerDeviceRemoved { which, .. } => {
              self.gamepads.remove(&which);
            },
            Event::ControllerAxisMotion { timestamp: _, which, axis, value } => {
              if self.is_trigger(&axis) {
                match self.to_trigger_event(&which, &axis, &value) {
                  Ok(adapted) => events.push(adapted),
                  Err(_) => ()
                }
              } else {
                match self.to_axis_event(&which, &axis, &value) {
                  Ok(adapted) => events.push(adapted),
                  Err(_) => ()
                }
              }
            },
            Event::ControllerButtonDown { timestamp: _, which, button } => {
              match self.to_button_event(&which, &button, true) {
                Ok(adapted) => events.push(adapted),
                Err(_) => ()
              }
            },
            Event::ControllerButtonUp {timestamp: _, which, button } => {
              match self.to_button_event(&which, &button, false) {
                Ok(adapted) => events.push(adapted),
                Err(_) => ()
              }
            },
            _ => ()
          },
          None => break
        },
        Err(_) => ()
      }
    }
    panic::set_hook(prev_hook); 
    return events;
  }
  
  fn is_connected(&mut self, gamepad_id: &usize) -> bool {
    return self.gamepads.contains_key(&(*gamepad_id as u32));
  }
}
