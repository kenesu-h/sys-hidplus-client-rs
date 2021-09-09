use crate::input::switch::SwitchPad;

use serde::{Serialize, Deserialize};

/**
 * A struct representing a configuration for a client. This is subject to
 * change, but:
 * - server_ip represents the IP of the target Switch.
 * - switch_pads represent what Switch controller type each slot will emulate.
 * - input_delays represent how much input delay each slot will have.
 * - left_deadzones represent the radius of the left deadzone for each slot.
 * - right_deadzones represent the radius of the right deadzone for each slot.
 */
#[derive(Serialize, Deserialize)]
pub struct Config {
  server_ip: String,
  switch_pads: Vec<SwitchPad>,
  input_delays: Vec<u8>,
  left_deadzones: Vec<f32>,
  right_deadzones: Vec<f32>
}

impl Default for Config {
  fn default() -> Config {
    return Config {
      server_ip: "".to_string(),
      switch_pads: c!(SwitchPad::ProController, for _i in 0..8),
      input_delays: c!(0, for _i in 0..8),
      left_deadzones: c!(0.0, for _i in 0..8),
      right_deadzones: c!(0.0, for _i in 0..8)
    }
  }
}

impl Config {
  pub fn new(
    server_ip: String, switch_pads: Vec<SwitchPad>, input_delays: Vec<u8>,
    left_deadzones: Vec<f32>, right_deadzones: Vec<f32>
  ) -> Config {
    return Config {
      server_ip: server_ip,
      switch_pads: switch_pads,
      input_delays: input_delays,
      left_deadzones: left_deadzones,
      right_deadzones: right_deadzones
    }
  }

  pub fn get_server_ip(&self) -> &String {
    return &self.server_ip;
  }

  pub fn get_switch_pads(&self) -> &Vec<SwitchPad> {
    return &self.switch_pads;
  }

  pub fn get_input_delays(&self) -> &Vec<u8> {
    return &self.input_delays;
  }

  pub fn get_left_deadzones(&self) -> &Vec<f32> {
    return &self.left_deadzones;
  }

  pub fn get_right_deadzones(&self) -> &Vec<f32> {
    return &self.right_deadzones;
  }
}
