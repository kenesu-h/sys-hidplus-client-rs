use crate::input::switch::SwitchPad;

use serde::{Serialize, Deserialize};

/**
 * A struct representing a configuration for a client.
 * - server_ip represents the IP of the target Switch.
 * - switch_pads represent what Switch controller type each slot will emulate.
 */
#[derive(Serialize, Deserialize)]
pub struct Config {
  server_ip: String,
  switch_pads: Vec<SwitchPad>,
  input_delays: Vec<u8>
  /*
  switch_pad_1: Option<SwitchPad>,
  input_delay_1: i8,
  switch_pad_2: Option<SwitchPad>,
  input_delay_2: i8,
  switch_pad_3: Option<SwitchPad>,
  input_delay_3: i8,
  switch_pad_4: Option<SwitchPad>,
  input_delay_4: i8
  */
}

impl Default for Config {
  fn default() -> Config {
    return Config {
      server_ip: "".to_string(),
      switch_pads: c!(SwitchPad::ProController, for _i in 0..4),
      input_delays: c!(0, for _i in 0..4)
      /*
      switch_pad_1: Some(SwitchPad::ProController),
      input_delay_1: 0,
      switch_pad_2: Some(SwitchPad::ProController),
      input_delay_2: 0,
      switch_pad_3: Some(SwitchPad::ProController),
      input_delay_3: 0,
      switch_pad_4: Some(SwitchPad::ProController),
      input_delay_4: 0
      */
    }
  }
}

impl Config {
  pub fn new(
    server_ip: String, switch_pads: Vec<SwitchPad>, input_delays: Vec<u8>
  ) -> Config {
    return Config {
      server_ip: server_ip,
      switch_pads: switch_pads,
      input_delays: input_delays
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
}
