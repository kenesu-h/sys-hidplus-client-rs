use crate::{
  config::Config,
  input::{
    adapter::common::{
      InputEvent
    },
    switch::{
      SwitchPad,
      EmulatedPad
    }
  }
};
use std::{
  net::UdpSocket,
  time
};

/**
 * A struct representing the main input client.
 * 
 * There's a lot that goes into a client, but the bare minimum is:
 * - A socket to communicate with the input server.
 * - The IP of the input server.
 *   - This must be preserved between update calls.
 * - A way to read inputs from a general gamepad API.
 * - A list of the emulated gamepads.
 *
 * We also need these, although the reasoning behind them might be more obscure:
 * - A way to read inputs from RawInput.
 *   - This is needed for XInput-incompatible gamepads and to possibly support
 *     4+ players.
 * - HashMaps mapping gamepad IDs to the index of their corresponding emulated
 *   gamepad.
 *   - This allows controller updates to be O(n) as opposed to O(n^2).
 */
pub struct ClientModel {
  server_ip: String,
  sock: UdpSocket,
  pads: Vec<EmulatedPad>,
}

impl ClientModel {
  /**
   * Constructs a new client from a config, and two input readers respectively
   * corresponding to general input APIs and RawInput.
   *
   * The socket itself is bound to port 8000, but no server IP is specified.
   * Empty input maps are initialized, as well as emulated gamepads with types
   * of None.
   */
  pub fn new() -> Result<ClientModel, String> {
    return match UdpSocket::bind("0.0.0.0:8000") {
      Ok(sock) => Ok(
        ClientModel {
          server_ip: "".to_string(),
          sock: sock,
          pads: c![EmulatedPad::new(), for _i in 0..4]
        }
      ),
      Err(e) => Err(format!("{}", e))
    }
  }

  pub fn get_server_ip(&self) -> &String {
    return &self.server_ip;
  }

  pub fn set_server_ip(&mut self, server_ip: &String) -> () {
    self.server_ip = server_ip.to_string();
  }

  // A method to return the number of emulated gamepads in this client model.
  pub fn num_pads(&self) -> usize {
    return self.pads.len();
  }

  // A method to update a gamepad in this client model to be disconnected.
  pub fn disconnect_pad(&mut self, i: &usize) -> () {
    self.pads[*i].disconnect();
  }

  // A method to update a gamepad in this client model using an input event.
  pub fn update_pad(&mut self, i: &usize, event: &InputEvent) -> () {
    self.pads[*i].update(event);
  }

  /**
   * A method to update a gamepad in this client model to be connected as a
   * given switch pad.
   */
  pub fn connect_pad(&mut self, i: &usize, switch_pad: &SwitchPad) -> () {
    self.pads[*i].connect(*switch_pad);
  }

  /**
   * A method that sends the current emulated pad states to the Switch.
   *
   * Like update_pads(), this should be called at a fixed time interval.
   */
  pub fn update_server(&self) -> Result<(), String> {
    match self.sock.send_to(
      &PackedData::new(&self.pads, 4).to_bytes(),
      format!("{}:8000", self.server_ip)
    ) {
      Err(e) => return Err(
        format!("The following error occurred: {}.", e)
      ),
      Ok(_) => Ok(())
    }
  }

  /**
   * A method disconnects all connected gamepads.
   *
   * This unfortunately uses a brute-force approach of disconnecting all the
   * gamepads, but there's no other way that doesn't involve modifying the
   * server. For now, a list of gamepads (all set to None) will be spammed over
   * the course of 3 seconds in order for shit to somehow stick onto the wall.
   * This hasn't failed so far, but this may change if a network happens to be
   * unstable.
   */
  pub fn cleanup(&mut self) -> Result<String, String> {
    println!("Cleaning up connected gamepads... This will take a moment.");
    self.pads = c![EmulatedPad::new(), for _i in 0..4];
    let start: time::Instant = time::Instant::now();
    while start.elapsed().as_millis() < 3000 {
      match self.sock.send_to(
        &PackedData::new(&self.pads, 4).to_bytes(),
        format!("{}:8000", self.server_ip)
      ) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => ()
      }
    }
    return Ok("Gamepads should now be cleaned up.".to_string());
  }
}

/**
 * A struct representing packed data to be sent to a Switch.
 * 
 * This isn't the cleanest or most dynamic thing by any means, but I wanted it
 * to be consistent with the original data structure.
 */
pub struct PackedData {
  magic: u16,
  connected: u16,

  con_type: u16,
  keys: u64,
  joy_l_x: i32,
  joy_l_y: i32,
  joy_r_x: i32,
  joy_r_y: i32,

  con_type2: u16,
  keys2: u64,
  joy_l_x2: i32,
  joy_l_y2: i32,
  joy_r_x2: i32,
  joy_r_y2: i32,

  con_type3: u16,
  keys3: u64,
  joy_l_x3: i32,
  joy_l_y3: i32,
  joy_r_x3: i32,
  joy_r_y3: i32,

  con_type4: u16,
  keys4: u64,
  joy_l_x4: i32,
  joy_l_y4: i32,
  joy_r_x4: i32,
  joy_r_y4: i32,
}

// Maps a switch pad (or lack thereof) to its integer counterpart.
fn to_switch_pad_value(switch_pad: &SwitchPad) -> i8 {
  return match switch_pad {
    SwitchPad::Disconnected => 0,
    SwitchPad::ProController => 1,
    SwitchPad::JoyConLSide => 2,
    SwitchPad::JoyConRSide => 3
  }
}

impl PackedData {
  // Constructs a packed data struct just from a list of pads.
  pub fn new(pads: &Vec<EmulatedPad>, connected: i8) -> PackedData {
    return PackedData {
      magic: 0x3276,
      connected: connected as u16,

      con_type: to_switch_pad_value(pads[0].get_switch_pad()) as u16,
      keys: *pads[0].get_keyout() as u64,
      joy_l_x: pads[0].get_left().0,
      joy_l_y: pads[0].get_left().1,
      joy_r_x: pads[0].get_right().0,
      joy_r_y: pads[0].get_right().1,

      con_type2: to_switch_pad_value(pads[1].get_switch_pad()) as u16,
      keys2: *pads[1].get_keyout() as u64,
      joy_l_x2: pads[1].get_left().0,
      joy_l_y2: pads[1].get_left().1,
      joy_r_x2: pads[1].get_right().0,
      joy_r_y2: pads[1].get_right().1,

      con_type3: to_switch_pad_value(pads[2].get_switch_pad()) as u16,
      keys3: *pads[2].get_keyout() as u64,
      joy_l_x3: pads[2].get_left().0,
      joy_l_y3: pads[2].get_left().1,
      joy_r_x3: pads[2].get_right().0,
      joy_r_y3: pads[2].get_right().1,

      con_type4: to_switch_pad_value(pads[3].get_switch_pad()) as u16,
      keys4: *pads[3].get_keyout() as u64,
      joy_l_x4: pads[3].get_left().0,
      joy_l_y4: pads[3].get_left().1,
      joy_r_x4: pads[3].get_right().0,
      joy_r_y4: pads[3].get_right().1,
    }
  }

  // Converts this packed data to structured bytes.
  pub fn to_bytes(&self) -> Vec<u8> {
    /* 
     * H - SwitchPad (Controller Type)
     * Q - Keyout
     * i - Stick Info 
     */
    structure!("<HHHQiiiiHQiiiiHQiiiiHQiiii").pack(
      self.magic,
      self.connected,

      self.con_type,
      self.keys,
      self.joy_l_x,
      self.joy_l_y,
      self.joy_r_x,
      self.joy_r_y,

      self.con_type2,
      self.keys2,
      self.joy_l_x2,
      self.joy_l_y2,
      self.joy_r_x2,
      self.joy_r_y2,

      self.con_type3,
      self.keys3,
      self.joy_l_x3,
      self.joy_l_y3,
      self.joy_r_x3,
      self.joy_r_y3,

      self.con_type4,
      self.keys4,
      self.joy_l_x4,
      self.joy_l_y4,
      self.joy_r_x4,
      self.joy_r_y4,
    ).unwrap()
  }
}
