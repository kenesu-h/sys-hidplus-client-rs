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
 * - manual_assign represents whether controllers are manually assigned using
 *   right bumper. When disabled, controllers will be automatically assigned
 * - anarchy_mode represents whether anarchy mode is enabled (all controllers
 *   control a single one).
 */
#[derive(Serialize, Deserialize)]
pub struct Config {
  server_ip: String,
  switch_pads: Vec<SwitchPad>,
  input_delays: Vec<u8>,
  left_deadzones: Vec<f32>,
  right_deadzones: Vec<f32>,
  manual_assign: bool,
  anarchy_mode: bool
}

impl Default for Config {
  fn default() -> Config {
    return Config {
      server_ip: "".to_string(),
      switch_pads: c!(SwitchPad::ProController, for _i in 0..8),
      input_delays: c!(0, for _i in 0..8),
      left_deadzones: c!(0.0, for _i in 0..8),
      right_deadzones: c!(0.0, for _i in 0..8),
      manual_assign: true,
      anarchy_mode: false
    }
  }
}

impl Config {
  pub fn new(
    server_ip: String, switch_pads: Vec<SwitchPad>, input_delays: Vec<u8>,
    left_deadzones: Vec<f32>, right_deadzones: Vec<f32>, manual_assign: bool,
    anarchy_mode: bool
  ) -> Config {
    return Config {
      server_ip: server_ip,
      switch_pads: switch_pads,
      input_delays: input_delays,
      left_deadzones: left_deadzones,
      right_deadzones: right_deadzones,
      manual_assign: manual_assign,
      anarchy_mode: anarchy_mode
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

  pub fn get_manual_assign(&self) -> &bool {
    return &self.manual_assign;
  }

  pub fn get_anarchy_mode(&self) -> &bool {
    return &self.anarchy_mode;
  }
}
