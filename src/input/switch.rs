use crate::input::adapter::common::{
  InputButton,
  InputAxis,
  InputEvent
};

use serde::{Serialize, Deserialize};

// An enum representing the different Switch controllers that can be emulated.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// An enum representing all the different buttons on a Switch controller.
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
  pub fn map_button(
    button: &InputButton, switch_pad: &SwitchPad
  ) -> Result<SwitchButton, String> {
    match button {
      InputButton::DPadUp => Ok(Self::DU),
      InputButton::DPadRight => Ok(Self::DR),
      InputButton::DPadDown => Ok(Self::DD),
      InputButton::DPadLeft => Ok(Self::DL),
      
      InputButton::LeftBumper => Ok(Self::L),
      InputButton::RightBumper => Ok(Self::R),
      // TODO: It may be worth changing these to SLL/SLR/SRL/SRR.
      InputButton::LeftTrigger => Ok(Self::ZL),
      InputButton::RightTrigger => Ok(Self::ZR),

      InputButton::Start => Ok(Self::Plus),
      InputButton::Select => Ok(Self::Minus),
      
      InputButton::North => match switch_pad {
        SwitchPad::Disconnected => Err(format!("No map for disconnected pad.")),
        SwitchPad::ProController => Ok(Self::X),
        SwitchPad::JoyConLSide => Ok(Self::DR),
        SwitchPad::JoyConRSide => Ok(Self::Y)
      },
      InputButton::East => match switch_pad {
        SwitchPad::Disconnected => Err(format!("No map for disconnected pad.")),
        SwitchPad::ProController => Ok(Self::A),
        SwitchPad::JoyConLSide => Ok(Self::DD),
        SwitchPad::JoyConRSide => Ok(Self::X)
      },
      InputButton::South => match switch_pad {
        SwitchPad::Disconnected => Err(format!("No map for disconnected pad.")),
        SwitchPad::ProController => Ok(Self::B),
        SwitchPad::JoyConLSide => Ok(Self::DL),
        SwitchPad::JoyConRSide => Ok(Self::A)
      },
      InputButton::West => match switch_pad {
        SwitchPad::Disconnected => Err(format!("No map for disconnected pad.")),
        SwitchPad::ProController => Ok(Self::Y),
        SwitchPad::JoyConLSide => Ok(Self::DU),
        SwitchPad::JoyConRSide => Ok(Self::B)
      }
    }
  }
}

/**
 * A struct representing an emulated Switch controller.
 * 
 * Emulated pads MUST contain:
 * - An integer representing the buttons pressed.
 * - Two tuples representing the states of the left and right analog sticks
 *   respectively.
 * 
 * Optionally they can have a Switch pad type and a reference to their
 * respective gamepad, since it's entirely possible for a pad to be initialized,
 * but not connected to anything.
 */
pub struct EmulatedPad {
  gamepad_id: Option<usize>,
  switch_pad: SwitchPad,
  keyout: i32,
  left: (i32, i32),
  right: (i32, i32)
}

impl EmulatedPad {
  /**
   * Constructs an emulated pad that is in a neutral state and isn't connected
   * to anything.
   */
  pub fn new() -> EmulatedPad {
    return EmulatedPad {
      gamepad_id: None,
      switch_pad: SwitchPad::Disconnected,
      keyout: 0,
      left: (0, 0),
      right: (0, 0)
    }
  }

  // Getters
  pub fn get_gamepad_id(&self) -> &Option<usize> {
    return &self.gamepad_id;
  }

  pub fn get_switch_pad(&self) -> &SwitchPad {
    return &self.switch_pad;
  }

  pub fn get_keyout(&self) -> &i32 {
    return &self.keyout;
  }

  pub fn get_left(&self) -> &(i32, i32) {
    return &self.left;
  }

  pub fn get_right(&self) -> &(i32, i32) {
    return &self.right;
  }

  // A method that connects this pad by assigning a gamepad ID and Switch pad.
  pub fn connect(&mut self, gamepad_id: &usize, switch_pad: SwitchPad) -> () {
    self.gamepad_id = Some(*gamepad_id);
    self.switch_pad = switch_pad;
  }

  /**
   * A method that disconnects this pad by removing its gamepad ID and Switch
   * pad, leaving both with values of None.
   */
  pub fn disconnect(&mut self) -> () {
    self.gamepad_id = None;
    self.switch_pad = SwitchPad::Disconnected;
  }

  // Attempts to update this pad using an input event.
  pub fn update(&mut self, event: &InputEvent) -> () {
    match event {
      InputEvent::GamepadButton(_, button, value) => {
        self.update_keyout(button, value)
      },
      InputEvent::GamepadAxis(_, axis, value) => self.update_axis(axis, value)
    }
  }

  // A helper method to update the keyout for a button.
  fn update_keyout(&mut self, button: &InputButton, value: &f32) -> () {
    if self.switch_pad != SwitchPad::Disconnected {
      match &SwitchButton::map_button(
        button,
        &self.switch_pad
      ) {
        Ok(switch_button) => self.set_del_bit(
          &switch_button.value(),
          &(*value as i32)
        ),
        Err(_) => ()
      }
    }
  }

  // A helper method to update the stick values for an axis.
  fn update_axis(&mut self, axis: &InputAxis, value: &f32) -> () {
    let converted: i32 = (*value * 32767.0) as i32;
    match axis {
      InputAxis::LeftX => self.left.0 = converted,
      InputAxis::LeftY => self.left.1 = converted,
      InputAxis::RightX => self.right.0 = converted,
      InputAxis::RightY => self.right.1 = converted
    }
  }

  /**
   * A helper method to update the keyout using a bitwise OR if an input value
   * isn't 0, otherwise a bitwise AND using the complement.
   */
  fn set_del_bit(&mut self, bit: &i32, value: &i32) -> () {
    if value != &0 {
      self.keyout = self.keyout | bit;
    } else {
      self.keyout = self.keyout & !bit;
    }
  }
}
