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
  switch_pad_1: Option<SwitchPad>,
  switch_pad_2: Option<SwitchPad>,
  switch_pad_3: Option<SwitchPad>,
  switch_pad_4: Option<SwitchPad>
}

impl Default for Config {
  fn default() -> Config {
    return Config {
      server_ip: "".to_string(),
      switch_pad_1: Some(SwitchPad::ProController),
      switch_pad_2: Some(SwitchPad::ProController),
      switch_pad_3: Some(SwitchPad::ProController),
      switch_pad_4: Some(SwitchPad::ProController)
    }
  }
}

impl Config { 
  pub fn get_server_ip(&self) -> &String {
    return &self.server_ip;
  }

  pub fn pads_to_vec(&self) -> Vec<Option<SwitchPad>> {
    return vec!(
      self.switch_pad_1,
      self.switch_pad_2,
      self.switch_pad_3,
      self.switch_pad_4
    );
  }
}
