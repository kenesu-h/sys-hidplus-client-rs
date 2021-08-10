use crate::{
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
 * Represents a model for an input client. The model is responsible for keeping
 * track of the emulated gamepads and sending their states over to the input
 * server - or in other words, a Nintendo Switch.
 */
pub struct ClientModel {
  server_ip: String,
  sock: UdpSocket,
  pads: Vec<EmulatedPad>,
}

impl ClientModel {
  /**
   * Constructs a model where the socket is bound to port 8000 and all emulated
   * gamepads are initially set to Disconnected. The server IP is also initially
   * blank as well, but this can be updated through its respective setter.
   */
  pub fn new() -> Result<ClientModel, String> {
    return match UdpSocket::bind("0.0.0.0:8000") {
      Ok(sock) => Ok(
        ClientModel {
          server_ip: "".to_string(),
          sock: sock,
          pads: c![EmulatedPad::new(), for _i in 0..8]
        }
      ),
      Err(e) => Err(format!("{}", e))
    }
  }

  // Server IP Getter
  pub fn get_server_ip(&self) -> &String {
    return &self.server_ip;
  }

  // Server IP Setter
  pub fn set_server_ip(&mut self, server_ip: &String) -> () {
    self.server_ip = server_ip.to_string();
  }

  // Returns the number of emulated gamepads in this model.
  pub fn num_pads(&self) -> usize {
    return self.pads.len();
  }

  /**
   * Disconnects a gamepad from this model.
   * 
   * This really just sets the target gamepad to a type of Disconnected; it
   * doesn't actually get rid of the gamepad itself.
   */
  pub fn disconnect_pad(&mut self, i: &usize) -> () {
    self.pads[*i].disconnect();
  }

  // Updates a target gamepad in this model using an input event.
  pub fn update_pad(&mut self, i: &usize, event: &InputEvent) -> () {
    self.pads[*i].update(event);
  }

  /**
   * Connects a gamepad to this model.
   *
   * Like disconnect_pad(), this just sets the target gamepad to the given
   * Switch pad type.
   */
  pub fn connect_pad(&mut self, i: &usize, switch_pad: &SwitchPad) -> () {
    self.pads[*i].connect(*switch_pad);
  }

  // Sends the current emulated pad states to the input server.
  pub fn update_server(&self) -> Result<(), String> {
    match self.sock.send_to(
      &PackedData::new(&self.pads, 8).to_bytes(),
      format!("{}:8000", self.server_ip)
    ) {
      Ok(_) => Ok(()),
      Err(e) => return Err(
        format!( 
          "The following error occurred while updating the server: {}.", e
        )
      )
    }
  }

  /**
   * Disconnects all connected gamepads through an unfortunately-brute-force
   * method.
   *
   * Without an established protocol, this seems to be the only way to
   * disconnect everything reliably.
   */
  pub fn cleanup(&mut self) -> Result<String, String> {
    for pad in &mut self.pads {
      pad.disconnect();
    }
    // self.pads = c![EmulatedPad::new(), for _i in 0..4];
    let start: time::Instant = time::Instant::now();
    while start.elapsed().as_millis() < 3000 {
      match self.sock.send_to(
        &PackedData::new(&self.pads, 8).to_bytes(),
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
 * Represents packed data to be sent to an input server.
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

  con_type5: u16,
  keys5: u64,
  joy_l_x5: i32,
  joy_l_y5: i32,
  joy_r_x5: i32,
  joy_r_y5: i32,

  con_type6: u16,
  keys6: u64,
  joy_l_x6: i32,
  joy_l_y6: i32,
  joy_r_x6: i32,
  joy_r_y6: i32,

  con_type7: u16,
  keys7: u64,
  joy_l_x7: i32,
  joy_l_y7: i32,
  joy_r_x7: i32,
  joy_r_y7: i32,

  con_type8: u16,
  keys8: u64,
  joy_l_x8: i32,
  joy_l_y8: i32,
  joy_r_x8: i32,
  joy_r_y8: i32,
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

      con_type5: to_switch_pad_value(pads[4].get_switch_pad()) as u16,
      keys5: *pads[4].get_keyout() as u64,
      joy_l_x5: pads[4].get_left().0,
      joy_l_y5: pads[4].get_left().1,
      joy_r_x5: pads[4].get_right().0,
      joy_r_y5: pads[4].get_right().1,

      con_type6: to_switch_pad_value(pads[5].get_switch_pad()) as u16,
      keys6: *pads[5].get_keyout() as u64,
      joy_l_x6: pads[5].get_left().0,
      joy_l_y6: pads[5].get_left().1,
      joy_r_x6: pads[5].get_right().0,
      joy_r_y6: pads[5].get_right().1,

      con_type7: to_switch_pad_value(pads[6].get_switch_pad()) as u16,
      keys7: *pads[6].get_keyout() as u64,
      joy_l_x7: pads[6].get_left().0,
      joy_l_y7: pads[6].get_left().1,
      joy_r_x7: pads[6].get_right().0,
      joy_r_y7: pads[6].get_right().1,

      con_type8: to_switch_pad_value(pads[7].get_switch_pad()) as u16,
      keys8: *pads[7].get_keyout() as u64,
      joy_l_x8: pads[7].get_left().0,
      joy_l_y8: pads[7].get_left().1,
      joy_r_x8: pads[7].get_right().0,
      joy_r_y8: pads[7].get_right().1,
    }
  }

  // Converts this packed data to structured bytes.
  pub fn to_bytes(&self) -> Vec<u8> {
    /* 
     * H - SwitchPad (Controller Type)
     * Q - Keyout
     * i - Stick Info 
     */
    structure!("<HHHQiiiiHQiiiiHQiiiiHQiiiiHQiiiiHQiiiiHQiiiiHQiiii").pack(
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

      self.con_type5,
      self.keys5,
      self.joy_l_x5,
      self.joy_l_y5,
      self.joy_r_x5,
      self.joy_r_y5,

      self.con_type6,
      self.keys6,
      self.joy_l_x6,
      self.joy_l_y6,
      self.joy_r_x6,
      self.joy_r_y6,

      self.con_type7,
      self.keys7,
      self.joy_l_x7,
      self.joy_l_y7,
      self.joy_r_x7,
      self.joy_r_y7,

      self.con_type8,
      self.keys8,
      self.joy_l_x8,
      self.joy_l_y8,
      self.joy_r_x8,
      self.joy_r_y8
    ).unwrap()
  }
}
