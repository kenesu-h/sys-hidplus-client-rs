use crate::{ 
  input::{
    adapter::common::{
      InputButton,
      InputEvent,
      InputAdapter
    },
    switch::{
      SwitchPad,
      EmulatedPad
    }
  },
  config::Config,
  model::ClientModel
};

use confy::ConfyError;
use std::{
  collections::{
    HashMap,
    HashSet
  }
};

/**
 * Represents a controller for an input client.
 *
 * Controllers are responsible for accepting user input (especially from
 * gamepads) and updating the model accordingly. The way user input is handled
 * can generally depend on configurations like input delays and deadzones.
 *
 * Controllers are also intended to be used by application structs as a means
 * to perform operations with the client, such as starting and connecting it
 * to a Switch, or changing configurations.
 */
pub struct ClientController {
  switch_pads: Vec<SwitchPad>,
  input_delays: Vec<u8>,
  left_deadzones: Vec<f32>,
  right_deadzones: Vec<f32>,

  model: ClientModel,
  running: bool,

  input_adapter: Box<dyn InputAdapter>,
  input_map: HashMap<usize, usize>,
  input_buffer: Vec<(InputEvent, u8)>
}

impl ClientController {
  /**
   * Constructs a controller from a model and a view, as well as an input
   * adapter. The input adapter is especially important in reading inputs from
   * supported gamepads.
   */
  pub fn new(
    model: ClientModel, input_adapter: Box<dyn InputAdapter>
  ) -> ClientController {
    return ClientController {
      switch_pads: vec!(),
      input_delays: vec!(),
      left_deadzones: vec!(),
      right_deadzones: vec!(),

      model: model,
      running: false,

      input_adapter: input_adapter,
      input_map: HashMap::new(),
      input_buffer: vec!()
    }
  }

  // A getter for the gamepads; you may need this for application structs.
  pub fn get_pads(&self) -> &Vec<EmulatedPad> {
    return self.model.get_pads();
  }

  // A bunch of setters, but they'll save the config every time they're used.
  pub fn set_server_ip(
    &mut self, server_ip: &String
  ) -> Result<String, String> {
    self.model.set_server_ip(server_ip);
    return self.save_config();
  }

  pub fn set_switch_pad(
    &mut self, i: &usize, switch_pad: &SwitchPad
  ) -> Result<String, String> {
    self.switch_pads[*i] = *switch_pad;
    return self.save_config();
  }

  pub fn set_input_delay(
    &mut self, i: &usize, input_delay: &u8
  ) -> Result<String, String> {
    self.input_delays[*i] = *input_delay;
    return self.save_config();
  }

  pub fn set_left_deadzone(
    &mut self, i: &usize, deadzone: &f32
  ) -> Result<String, String> {
    self.left_deadzones[*i] = *deadzone;
    return self.save_config();
  }

  pub fn set_right_deadzone(
    &mut self, i: &usize, deadzone: &f32
  ) -> Result<String, String> {
    self.right_deadzones[*i] = *deadzone;
    return self.save_config();
  }

  /**
   * Initializes this controller with a greeting message, and by loading a
   * config.
   */
  pub fn initialize(&mut self) -> Result<String, String> {
    return match self.load_config() {
      Ok(o) => {
        return Ok(format!("{} Welcome to sys-hidplus-client-rs!", o));
      },
      Err(e) => Err(e)
    }
  }

  // Loads a config, which is expected to be in the current directory.
  fn load_config(&mut self) -> Result<String, String> {
    let confy_load: Result<Config, ConfyError> =
      confy::load_path("./config.toml");
    return match confy_load {
      Ok(config) => {
        self.model.set_server_ip(config.get_server_ip());
        self.switch_pads = config.get_switch_pads().clone();
        self.input_delays = config.get_input_delays().clone();
        self.left_deadzones = config.get_left_deadzones().clone();
        self.right_deadzones = config.get_right_deadzones().clone();
        return Ok("Config successfully loaded.".to_string());
      },
      Err(e) => Err(
        format!("Error occurred while loading config: {}", e)
      )
    }
  } 

  // Saves the current config to the current directory.
  fn save_config(&self) -> Result<String, String> {
    return match confy::store_path("./config.toml", self.current_config())  {
      Ok(_) => Ok("Config successfully saved.".to_string()),
      Err(e) => Err(
        format!("Error occurred while saving config: {}", e)
      )
    }
  }

  // Returns the current config as an effective clone.
  fn current_config(&self) -> Config {
    return Config::new(
      self.model.get_server_ip().to_string(),
      self.switch_pads.clone(),
      self.input_delays.clone(),
      self.left_deadzones.clone(),
      self.right_deadzones.clone()
    );
  }

  // Restarts the client, but only if it's currently running.
  pub fn restart(&mut self) -> Result<String, String> {
    if self.running {
      return match self.stop() {
        Ok(_) => self.start(),
        Err(e) => Err(e)
      }
    } else {
      return Err(
        "The client needs to be running in order to restart it.".to_string()
      );
    }
  }

  // Starts the client if it's not running yet.
  pub fn start(&mut self) -> Result<String, String> {
    if self.model.get_server_ip().is_empty() {
      return Err(
        "The server_ip field in config.toml is empty! If this is your first \
        time running the client, please set it to the IP of your Switch. You \
        can use 'set_server_ip `server_ip`', where `server_ip` is replaced with
        your Switch's IP. Be sure not to include the single quotes too. If you
        need an example, type 'help set_server_ip'."
        .to_string()
      );
    } else {
      if self.running {
        return Err(String::from("The client is already running."));
      } else {
        self.running = true;
        return Ok(String::from("The client is ready to receive inputs."));
      }
    }
  }

  // Stops the client if it's currently running.
  pub fn stop(&mut self) -> Result<String, String> {
    if self.running {
      self.running = false;
      match self.cleanup() {
        Ok(_) => return Ok("The client has been stopped.".to_string()),
        Err(e) => return Err(e)
      }
    } else {
      return Err("The client isn't running.".to_string());
    }
  }

  /**
   * Cleans up and disconnects all connected gamepads. If none are connected,
   * there's no need to tell the model to clean up the Switch as well.
   */
  fn cleanup(&mut self) -> Result<String, String> {
    if self.input_map.len() > 0 {
      self.input_map.clear();
      return self.model.cleanup();
    } else {
      return Ok(
        String::from("No need to clean up since no gamepads are connected.")
      );
    }
  }

  /**
   * Preps the client for an exit by first stopping the client if it is
   * currently running. This method then returns whether it's okay to
   * successfully exit. Unsuccessfully exiting entails an error occurring during
   * the stopping process.
   *
   * While we could just directly exit here, sometimes application structs have
   * things to clean up to ensure... well, a clean exit.
   */
  pub fn exit_prep(&mut self) -> Result<(), String> {
    if self.running {
      return match self.stop() {
        Ok(_) => Ok(()),
        Err(e) => Err(
          format!(
            "Failed to exit successfully. The following error occurred: {}", e
          )
        )
      }
    } else {
      return Ok(());
    }
  }

  /**
   * Updates this controller, which then updates the model accordingly. This is
   * also where input events are received and parsed.
   *
   * This should be used at a fixed time interval.
   */
  pub fn update(&mut self) -> Vec<Result<String, String>> {
    let mut results: Vec<Result<String, String>> = vec!();
    if self.running { 
      results.extend(self.update_inputs());
      results.push(self.update_server());
    }
    return results;
  }

  /**
   * Tells the model to update the input server. If an issue occurs while doing
   * so, the client will attempt to stop and cleanup immediately.
   */
  fn update_server(&mut self) -> Result<String, String> {
    let mut errors: Vec<String> = vec!();
    match self.model.update_server() {
      Ok(_) => return Ok(String::new()),
      Err(e) => {
        errors.push(e);
        if let Err(e_stop) = self.stop() {
          errors.push(e_stop);
        }
        return Err(
          format!(
            "Received the following errors while attempting to update and \
            cleanup the server: {:?}",
            errors
          )
        );
      }
    }
  }

  // Update everything related to inputs on this controller.
  fn update_inputs(&mut self) -> Vec<Result<String, String>> {
    let mut results: Vec<Result<String, String>> = vec!();
    results.extend(self.disconnect_inactive());
    self.fill_input_buffer();
    results.extend(self.parse_input_buffer());
    return results;
  } 

  // Attempts to disconnect all disconnected gamepads from this controller.
  fn disconnect_inactive(&mut self) -> Vec<Result<String, String>> {
    let mut results: Vec<Result<String, String>> = vec!();
    for (gamepad_id, _) in self.input_map.clone() {
      if !self.input_adapter.is_connected(&gamepad_id) {
        results.push(self.disconnect(&gamepad_id));
      }
    }
    return results;
  }

  // Disconnects the gamepad with the given ID, if it exists.
  fn disconnect(&mut self, gamepad_id: &usize) -> Result<String, String> {
    if self.input_map.contains_key(gamepad_id) {
      let i: usize = *self.input_map.get(gamepad_id).unwrap();
      self.input_map.remove(gamepad_id);
      self.model.disconnect_pad(&i);
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

  // Fills the input buffer with events from the input adapter.
  fn fill_input_buffer(&mut self) -> () {
    for event in self.input_adapter.read() {
      // Apply artificial input lag only if it's a connected controller.
      if let Some(i) = self.input_map.get(event.get_gamepad_id()) {
        self.input_buffer.insert(
          0,
          (event, self.input_delays[*i]),
        );
      } else {
        self.input_buffer.insert(0, (event, 0));
      }
    }
  }

  // Parses events from the input buffer and updates all corresponding gamepads.
  fn parse_input_buffer(&mut self) -> Vec<Result<String, String>> {
    let mut results: Vec<Result<String, String>> = vec!();
    let mut new_buffer: Vec<(InputEvent, u8)> = vec!();
    
    while let Some((event, delay)) = self.input_buffer.pop() {
      if delay == 0 {
        if let Some(i) = self.input_map.get(event.get_gamepad_id()) {
          self.model.update_pad(
            &i,
            &event,
            &(self.left_deadzones[*i], self.right_deadzones[*i])
          );
        } else {
          if let InputEvent::GamepadButton(gamepad_id, button, value) = event {
            if button == InputButton::RightBumper && value == 1 {
              results.push(self.connect(&gamepad_id));
            }
          }
        }
      } else {
        new_buffer.insert(0, (event, delay - 1));
      }
    }

    self.input_buffer = new_buffer;
    return results;
  }

  /**
   * Attempts to assign the given gamepad ID and switch pad type to an open
   * slot, while mapping said ID the corresponding index. Slots are open so as
   * long as they are not equal to a type of None, or if the associated
   * controller is reported by the input adapter as disconnected.
   *
   * This is O(n^2) when used in the context of parse_input_buffer(), but at
   * least controller assignment doesn't happen often.
   */
  fn connect(&mut self, gamepad_id: &usize) -> Result<String, String> {
    let mut mapped: HashSet<&usize> = HashSet::new();
    for value in self.input_map.values() {
      mapped.insert(value);
    }
    for i in 0..self.model.num_pads() {
      if !mapped.contains(&i) {
        let switch_pad: SwitchPad = self.switch_pads[i];
        if switch_pad != SwitchPad::Disconnected {
          self.input_map.insert(*gamepad_id, i);
          self.model.connect_pad(&i, &switch_pad);
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
}