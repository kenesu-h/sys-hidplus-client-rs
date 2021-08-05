use crate::{
  config::Config,
  model::ClientModel,
  view::common::ClientView,
  input::{
    adapter::common::{
      InputButton,
      InputEvent,
      InputAdapter
    },
    switch::SwitchPad
  }
};

use confy::ConfyError;
use std::{
  collections::{
    HashMap,
    HashSet
  },
  process,
  str::FromStr
};

pub struct ClientController {
  switch_pads: Vec<SwitchPad>,
  input_delays: Vec<u8>,

  model: ClientModel,
  view: Box<dyn ClientView>,
  running: bool,

  input_adapter: Box<dyn InputAdapter>,
  input_map: HashMap<usize, usize>,
  input_buffer: Vec<(InputEvent, u8)>,
}

impl ClientController {
  pub fn new(
    model: ClientModel, view: Box<dyn ClientView>,
    input_adapter: Box<dyn InputAdapter>
  ) -> ClientController {
    return ClientController {
      switch_pads: vec!(),
      input_delays: vec!(),

      model: model,
      view: view,
      running: false,

      input_adapter: input_adapter,
      input_map: HashMap::new(),
      input_buffer: vec!()
    }
  }

  pub fn initialize(&mut self) -> Result<(), String> {
    return match self.load_config() {
      Ok(msg) => {
        self.view.writeln(msg);
        self.view.writeln(
          "Welcome to sys-hidplus-client-rs! Type 'start' to begin the client \
          or 'exit' to close it. Type 'help' for a list of all available \
          commands."
          .to_string()
        );
        return Ok(());
      },
      Err(e) => Err(e)
    }
  }

  /**
   * Setters, but these fields should only be set (outside of the controller) by
   * actually sending commands to the controller.
   */
  fn set_server_ip(&mut self, server_ip: &String) -> Result<String, String> {
    self.model.set_server_ip(server_ip);
    return self.save_config();
  }

  fn set_switch_pad(
    &mut self, i: &usize, switch_pad: &SwitchPad
  ) -> Result<String, String> {
    self.switch_pads[*i] = *switch_pad;
    return self.save_config();
  }

  fn set_input_delay(
    &mut self, i: &usize, input_delay: &u8
  ) -> Result<String, String> {
    self.input_delays[*i] = *input_delay;
    return self.save_config();
  }

  pub fn load_config(&mut self) -> Result<String, String> {
    let confy_load: Result<Config, ConfyError> =
      confy::load_path("./config.toml");
    return match confy_load {
      Ok(config) => {
        self.model.set_server_ip(config.get_server_ip());
        self.switch_pads = config.get_switch_pads().clone();
        self.input_delays = config.get_input_delays().clone();
        return Ok("Config successfully loaded.".to_string());
      },
      Err(e) => Err(
        format!("Error occurred while loading config: {}", e)
      )
    }
  } 

  pub fn save_config(&self) -> Result<String, String> {
    return match confy::store_path("./config.toml", self.current_config())  {
      Ok(_) => Ok("Config successfully saved.".to_string()),
      Err(e) => Err(
        format!("Error occurred while saving config: {}", e)
      )
    }
  }

  fn current_config(&self) -> Config {
    return Config::new(
      self.model.get_server_ip().to_string(),
      self.switch_pads.clone(),
      self.input_delays.clone()
    );
  }

  fn start(&mut self) -> Result<String, String> {
    if self.model.get_server_ip().is_empty() {
      return Err(
        "The server_ip field in config.toml is empty! If this is your first \
        time running the client, please set it to the IP of your Switch."
        .to_string()
      );
    } else {
      if self.running {
        return Err("The client is already running.".to_string());
      } else {
        self.running = true;
        return Ok("The client is ready to receive inputs.".to_string());
      }
    }
  }

  fn stop(&mut self) -> Result<String, String> {
    if self.running {
      self.running = false;
      match self.cleanup() {
        Ok(_) => return Ok(format!("The client has been stopped.")),
        Err(e) => return Err(e)
      }
    } else {
      return Err(format!("The client isn't running."));
    }
  }

  fn exit(&mut self) -> Result<String, String> {
    if self.running {
      match self.stop() {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1)
      }
    } else {
      process::exit(0);
    }
  }

  pub fn update(&mut self) -> () {
    match self.view.update() {
      Ok(_) => (),
      Err(e) => {
        println!("{}", e);
        self.exit();
      }
    }
    self.parse_command_buffer();

    if self.running { 
      self.update_inputs();
      match self.update_server() {
        Ok(_) => (),
        Err(e) => self.view.writeln(format!("{}", e))
      }
    }
  }

  fn update_inputs(&mut self) -> () {
    self.disconnect_inactive();
    self.fill_input_buffer();
    self.parse_input_buffer();
  }

  fn update_server(&mut self) -> Result<(), String> {
    let mut errors: Vec<String> = vec!();
    match self.model.update_server() {
      Ok(msg) => return Ok(msg),
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

  pub fn cleanup(&mut self) -> Result<String, String> {
    self.input_map.clear();
    return self.model.cleanup();
  }

  fn disconnect_inactive(&mut self) -> () {
    for (gamepad_id, _) in self.input_map.clone() {
      if !self.input_adapter.is_connected(&gamepad_id) {
        match self.disconnect(&gamepad_id) {
          Ok(msg) => self.view.writeln(format!("{}", msg)),
          Err(e) => self.view.writeln(format!("{}", e))
        }
      }
    }
  }

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

  /**
   * A helper method to fill the input buffer with events from the input
   * adapter.
   */
  fn fill_input_buffer(&mut self) -> () {
    for event in self.input_adapter.read() {
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

  /**
   * A helper method that parses events from the input buffer and updates
   * corresponding gamepads.
   */
  fn parse_input_buffer(&mut self) -> () {
    let mut new_buffer: Vec<(InputEvent, u8)> = vec!();
    while let Some((event, delay)) = self.input_buffer.pop() {
      if delay == 0 {
        if let Some(i) = self.input_map.get(event.get_gamepad_id()) {
          self.model.update_pad(&i, &event);
        } else {
          if let InputEvent::GamepadButton(gamepad_id, button, value) = event {
            if button == InputButton::RightBumper && value == 1.0 {
              match self.connect(&gamepad_id) {
                Ok(msg) => self.view.writeln(format!("{}", msg)),
                Err(e) => self.view.writeln(format!("{}", e))
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

  pub fn parse_command_buffer(&mut self) -> () {
    while let Some(command) = self.view.get_command_buffer().pop() {
      let parts: Vec<&str> = command.split(" ").collect::<Vec<&str>>();
      match self.parse_command(&parts[0], &parts[1..]) {
        Ok(msg) => self.view.writeln(format!("{}", msg)),
        Err(e) => self.view.writeln(format!("{}", e))
      }
    }
  }

  fn parse_command(
    &mut self, keyword: &str, args: &[&str]
  ) -> Result<String, String> {
    return match keyword {
      "start" => self.start(),
      "stop" => self.stop(),
      "exit" => self.exit(),
      "set_server_ip" => {
        if args.len() >= 1 {
          return self.set_server_ip(&args[0].to_string());
        } else {
          return Err(
            "Usage: set_server_ip server_ip\n
            \n
            Example, if your Switch's IP is 192.168.1.199:\n
            set_server_ip 192.168.1.199"
            .to_string()
          );
        }
      },
      "set_switch_pad" => {
        if args.len() >= 2 {
          if let Ok(i) = args[0].parse::<usize>() {
            if let Ok(switch_pad) = SwitchPad::from_str(args[1]) {
              return self.set_switch_pad(&i, &switch_pad);
            }
          }
        }
        return Err(
          "Usage: set_switch_pad i switch_pad\n
          \n
          i must be either 0 or a positive integer. It also represents the \
          target index: slot numbers are always equal to i + 1.\n
          switch_pad must be one of: Disconnected, ProController, JoyConLSide, \
          JoyConRSide.\n
          \n
          Example, if you want to set the controller in slot 2 to a sideways \
          left JoyCon:\n
          set_switch_pad 1 JoyConLSide"
          .to_string()
        );
      },
      "set_input_delay" => {
        if args.len() >= 2 {
          if let Ok(i) = args[0].parse::<usize>() {
            if let Ok(input_delay) = args[1].parse::<u8>() {
              return self.set_input_delay(&i, &input_delay);
            }
          }
        }
        return Err(
          "Usage: set_input_delay i input_delay\n
          \n
          i must be either 0 or a positive integer. It also represents the \
          target index: slot numbers are always equal to i + 1.\n
          input_delay must be either 0 or a positive integer less than 256.\n
          \n
          Example, if you want to set the input delay of the controller in \
          slot 3 to 6 frames:\n
          set_input_delay 2 6"
          .to_string()
        );
      },
      _ => Err(format!("{} is not a valid command.", keyword))
    }
  }
}


