use crate::input::adapter::common::{
  InputButton,
  InputAxis,
  InputEvent
};

use serde::{Serialize, Deserialize};
use strum_macros::EnumString;

// Represents the different Switch controllers that can be emulated.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, EnumString)]
pub enum SwitchPad {
  Disconnected,
  ProController,
  JoyConLSide,
  JoyConRSide,

  /* TO BE ADDED:
  JoyConLR,
  JoyConL,
  JoyConR
  */
}

// Represents all the different buttons on a Switch controller.
#[derive(Debug, PartialEq, EnumString)]
pub enum SwitchButton {
  A,
  B,
  X,
  Y,
  LST,
  RST,
  L,
  R,
  ZL,
  ZR,
  Plus,
  Minus,
  DL,
  DU,
  DR,
  DD,
  LL,
  LU,
  LR,
  LD,
  RL,
  RU,
  RR,
  RD,
  SLL,
  SRL,
  SLR,
  SRR
}

impl SwitchButton {
  // Returns the bit corresponding to this button.
  pub fn value(&self) -> i32 {
    // TODO: Home button cannot be emulated until libnx adds support.
    match self {
      Self::A => return 1,
      Self::B => return 1 << 1,
      Self::X => return 1 << 2,
      Self::Y => return 1 << 3,
      Self::LST => return 1 << 4,
      Self::RST => return 1 << 5,
      Self::L => return 1 << 6,
      Self::R => return 1 << 7,
      Self::ZL => return 1 << 8,
      Self::ZR => return 1 << 9,
      Self::Plus => return 1 << 10,
      Self::Minus => return 1 << 11,
      Self::DL => return 1 << 12,
      Self::DU => return 1 << 13,
      Self::DR => return 1 << 14,
      Self::DD => return 1 << 15,
      Self::LL => return 1 << 16,
      Self::LU => return 1 << 17,
      Self::LR => return 1 << 18,
      Self::LD => return 1 << 19,
      Self::RL => return 1 << 20,
      Self::RU => return 1 << 21,
      Self::RR => return 1 << 22,
      Self::RD => return 1 << 23,
      Self::SLL => return 1 << 24,
      Self::SRL => return 1 << 25,
      Self::SLR => return 1 << 26,
      Self::SRR => return 1 << 27
    }
  }

  /**
   * Maps an input event button to a Switch button depending on the specified
   * pad type.
   */
  // TODO: Make this neater later.
  pub fn map_button(
    button: &InputButton, switch_pad: &SwitchPad
  ) -> Result<SwitchButton, String> {
    match button {
      InputButton::DPadUp => Ok(Self::DU),
      InputButton::DPadRight => Ok(Self::DR),
      InputButton::DPadDown => Ok(Self::DD),
      InputButton::DPadLeft => Ok(Self::DL),
     
      InputButton::LeftStick => Ok(Self::LST),
      InputButton::RightStick => Ok(Self::RST),

      InputButton::LeftBumper => Ok(Self::L),
      InputButton::RightBumper => Ok(Self::R),
      InputButton::LeftTrigger => Ok(Self::ZL),
      InputButton::RightTrigger => Ok(Self::ZR),

      InputButton::Start => Ok(Self::Plus),
      InputButton::Select => Ok(Self::Minus),
      
      InputButton::North => match switch_pad {
        SwitchPad::Disconnected => Err(
          String::from("No map for disconnected pad.")
        ),
        SwitchPad::ProController => Ok(Self::X),
        SwitchPad::JoyConLSide => Ok(Self::DR),
        SwitchPad::JoyConRSide => Ok(Self::Y)
      },
      InputButton::East => match switch_pad {
        SwitchPad::Disconnected => Err(
          String::from("No map for disconnected pad.")
        ),
        SwitchPad::ProController => Ok(Self::A),
        SwitchPad::JoyConLSide => Ok(Self::DD),
        SwitchPad::JoyConRSide => Ok(Self::X)
      },
      InputButton::South => match switch_pad {
        SwitchPad::Disconnected => Err(
          String::from("No map for disconnected pad.")
        ),
        SwitchPad::ProController => Ok(Self::B),
        SwitchPad::JoyConLSide => Ok(Self::DL),
        SwitchPad::JoyConRSide => Ok(Self::A)
      },
      InputButton::West => match switch_pad {
        SwitchPad::Disconnected => Err(
          String::from("No map for disconnected pad.")
        ),
        SwitchPad::ProController => Ok(Self::Y),
        SwitchPad::JoyConLSide => Ok(Self::DU),
        SwitchPad::JoyConRSide => Ok(Self::B)
      },
      InputButton::Guide => Err(String::from("No map for the guide button."))
    }
  }
}

/**
 * Represents an emulated Switch controller.
 *
 * The buttons pressed on a emulated pad are represented through a "keyout"
 * field, which is updated through bitwise operations. This could be done
 * through a map of buttons to booleans instead, but this is more true to the
 * original client and, in all honesty, is way more compact.
 */
#[derive(Copy, Clone)]
pub struct EmulatedPad {
  switch_pad: SwitchPad,
  keyout: i32,
  left: (i16, i16),
  right: (i16, i16)
}

impl EmulatedPad {
  /**
   * Constructs an emulated pad that is in a neutral state and isn't connected
   * to anything.
   */
  pub fn new() -> EmulatedPad {
    return EmulatedPad {
      switch_pad: SwitchPad::Disconnected,
      keyout: 0,
      left: (0, 0),
      right: (0, 0)
    }
  }

  // Getters
  pub fn get_switch_pad(&self) -> &SwitchPad {
    return &self.switch_pad;
  }

  pub fn get_keyout(&self) -> &i32 {
    return &self.keyout;
  }

  pub fn get_left(&self) -> &(i16, i16) {
    return &self.left;
  }

  pub fn get_right(&self) -> &(i16, i16) {
    return &self.right;
  }

  // Connects this pad by assigning a Switch pad.
  pub fn connect(&mut self, switch_pad: SwitchPad) -> () {
    self.switch_pad = switch_pad;
  }

  // Disconnects this pad by setting its Switch pad type to Disconnected.
  pub fn disconnect(&mut self) -> () {
    self.switch_pad = SwitchPad::Disconnected;
  }

  // Updates this pad using an input event.
  pub fn update(&mut self, event: &InputEvent) -> () {
    match event {
      InputEvent::GamepadButton(_, button, value) => {
        self.update_keyout(button, value)
      },
      InputEvent::GamepadAxis(_, axis, value) => self.update_axis(axis, value)
    }
  }

  pub fn is_pressed(&self, button: &SwitchButton) -> bool {
    return (&self.keyout & button.value()) != 0
  }

  // Updates this pad's keyout.
  fn update_keyout(&mut self, button: &InputButton, value: &i16) -> () {
    if self.switch_pad != SwitchPad::Disconnected {
      match &SwitchButton::map_button(
        button,
        &self.switch_pad
      ) {
        Ok(switch_button) => self.set_del_bit(
          &switch_button.value(),
          &value
        ),
        Err(_) => ()
      }
    }
  }

  // Updates the stick values for an axis.
  fn update_axis(&mut self, axis: &InputAxis, value: &i16) -> () {
    match axis {
      InputAxis::LeftX => self.left.0 = *value,
      InputAxis::LeftY => self.left.1 = *value,
      InputAxis::RightX => self.right.0 = *value,
      InputAxis::RightY => self.right.1 = *value
    }
  }

  /**
   * Updates the keyout using a bitwise OR if an input value isn't 0, otherwise
   * a bitwise AND using the complement.
   */
  fn set_del_bit(&mut self, bit: &i32, value: &i16) -> () {
    if value != &0 {
      self.keyout = self.keyout | bit;
    } else {
      self.keyout = self.keyout & !bit;
    }
  }
}
