use crate::{
  config::Config,
  input::{
    adapter::common::{
      InputButton,
      InputEvent,
      InputAdapter,
    },
    switch::{
      SwitchPad,
      EmulatedPad
    }
  }
};
use std::{
  collections::{
    HashMap,
    HashSet
  },
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
pub struct Client {
  config: Config,
  sock: UdpSocket,

  input_adapter: Box<dyn InputAdapter>,
  input_map: HashMap<usize, usize>,
  input_buffer: Vec<(InputEvent, i8)>,

  pads: Vec<EmulatedPad>,
}

impl Client {
  /**
   * Constructs a new client from a config, and two input readers respectively
   * corresponding to general input APIs and RawInput.
   *
   * The socket itself is bound to port 8000, but no server IP is specified.
   * Empty input maps are initialized, as well as emulated gamepads with types
   * of None.
   */
  pub fn new(
    config: Config,
    input_adapter: Box<dyn InputAdapter>
  ) -> Result<Client, String> {
    if config.get_server_ip().is_empty() {
      return Err(
        format!(
          "The server_ip field in config.toml is empty! If this is your first \
          time running the client, please open it using a text editor and fill \
          it out with the IP of your Switch. For example, if the IP is \
          192.168.1.199, the server_ip field should look like this: \
          server_ip = '192.168.1.199'"
        )
      );
    } else {
      return match UdpSocket::bind("0.0.0.0:8000") {
        Ok(sock) => Ok(
          Client {
            config: config,
            sock: sock,
            input_adapter: input_adapter,
            input_map: HashMap::new(),
            input_buffer: vec!(),
            pads: c![EmulatedPad::new(), for _i in 0..4]
          }
        ),
        Err(e) => Err(format!("{}", e))
      }
    }
  } 

  /**
   * A method that updates all emulated gamepads, disconnecting any unconnected
   * gamepads and parses input adapter events. Should be called at a fixed time
   * interval.
   */
  pub fn update_pads(&mut self) -> () {
    self.disconnect_inactive();
    self.fill_buffer();
    self.parse_buffer();
  }

  // A helper method that disconnects any gamepads that aren't connected.
  fn disconnect_inactive(&mut self) -> () {
    for (gamepad_id, _) in self.input_map.clone() {
      if !self.input_adapter.is_connected(&gamepad_id) {
        match self.disconnect(&gamepad_id) {
          Ok(msg) => println!("{}", msg),
          Err(e) => println!("{}", e)
        }
      }
    }
  }

  fn disconnect(&mut self, gamepad_id: &usize) -> Result<String, String> {
    if self.input_map.contains_key(gamepad_id) {
      let i: usize = *self.input_map.get(gamepad_id).unwrap();
      self.input_map.remove(gamepad_id);
      self.pads[i].disconnect();
      return Ok(
        format!(
          "Disconnected gamepad (id: {}) from slot {}.",
          gamepad_id,
          i + 1
        )
      ); 
    } else {
      return Err(
        format!(
          "No gamepad with an id of {} is connected.",
          gamepad_id
        )
      );
    }
  } 

  /**
   * A helper method to fill the input buffer with events from the input
   * adapter.
   */
  fn fill_buffer(&mut self) -> () {
    for event in self.input_adapter.read() {
      if let Some(i) = self.input_map.get(event.get_gamepad_id()) {
        self.input_buffer.insert(
          0,
          (event, self.config.get_input_delays()[*i]),
        );
      } else {
        self.input_buffer.insert(0, (event, 0));
      }
    }
  }

  /**
   * A helper method that parses events from the input buffer and updates
   * corresponding gamepads.
   */
  fn parse_buffer(&mut self) -> () {
    let mut new_buffer: Vec<(InputEvent, i8)> = vec!();
    while let Some((event, delay)) = self.input_buffer.pop() {
      if delay == 0 {
        if let Some(i) = self.input_map.get(event.get_gamepad_id()) {
          self.pads[*i].update(&event);
        } else {
          if let InputEvent::GamepadButton(gamepad_id, button, value) = event {
            if button == InputButton::RightBumper && value == 1.0 {
              match self.connect(&gamepad_id) {
                Ok(msg) => println!("{}", msg),
                Err(e) => println!("{}", e)
              }
            }
          }
        }
      } else {
        new_buffer.insert(0, (event, delay - 1));
      }
    }
    self.input_buffer = new_buffer;
  }

  /**
   * A helper method that attempts to assign the given gamepad ID and switch pad
   * type to an open slot, while mapping said ID the corresponding index. Slots
   * are open so as long as they are not equal to None, or if the associated
   * controller is reported by the respective input reader as disconnected.
   *
   * Is O(n^2) in the context of parse_buffer(), but at least controller
   * assignment shouldn't happen often.
   */
  fn connect(&mut self, gamepad_id: &usize) -> Result<String, String> {
    let mut mapped: HashSet<&usize> = HashSet::new();
    for value in self.input_map.values() {
      mapped.insert(value);
    }
    for i in 0..self.pads.len() {
      if !mapped.contains(&i) {
        let switch_pad: SwitchPad = self.config.get_switch_pads()[i];
        if switch_pad != SwitchPad::Disconnected {
          self.input_map.insert(*gamepad_id, i);
          self.pads[i].connect(gamepad_id, switch_pad);
          return Ok(
            format!(
              "Gamepad (id: {}) connected to slot {}.",
              &gamepad_id,
              i + 1
            )
          );
        }
      }
    }
    return Err(
      format!(
        "Couldn't connect gamepad (id: {}) since no slots are available.",
        gamepad_id
      )
    );
  }

  /**
   * A method that sends the current emulated pad states to the Switch.
   *
   * Like update_pads(), this should be called at a fixed time interval.
   */
  pub fn update_server(&self) -> Result<(), String> {
    match self.sock.send_to(
      &PackedData::new(&self.pads, 4).to_bytes(),
      format!("{}:8000", self.config.get_server_ip())
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
        format!("{}:8000", self.config.get_server_ip())
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
