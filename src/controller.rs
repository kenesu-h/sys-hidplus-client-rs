use crate::{ 
  input::{
    adapter::common::{
      InputButton,
      InputEvent,
      InputAdapter
    },
    switch::SwitchPad
  },
  config::Config,
  model::ClientModel,
  view::common::ClientView 
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

/**
 * Represents a controller for an input client. The controller is ultimately
 * responsible for accepting user input (especially from gamepads), as well as
 * updating the model and view accordingly. This also means it's responsible for
 * mapping gamepads to individual slots.
 */
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
  /**
   * Constructs a controller from a model and a view, as well as an input
   * adapter. The input adapter is especially important in reading inputs from
   * supported gamepads.
   */
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

  /**
   * Setters, but these fields should only be set (outside of the controller) by
   * actually sending commands to the controller. Every time a setter is called,
   * the current config is saved.
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

  /**
   * Initializes this controller with a greeting message, and by loading a
   * config.
   */
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

  // Loads a config, which is expected to be in the current directory.
  fn load_config(&mut self) -> Result<String, String> {
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
      self.input_delays.clone()
    );
  }

  // Restarts the client, but only if it's currently running.
  fn restart(&mut self) -> Result<String, String> {
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
  fn start(&mut self) -> Result<String, String> {
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
        return Err("The client is already running.".to_string());
      } else {
        self.running = true;
        return Ok("The client is ready to receive inputs.".to_string());
      }
    }
  }

  // Stops the client if it's currently running.
  fn stop(&mut self) -> Result<String, String> {
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

  // Cleans up and disconnects all connected gamepads.
  fn cleanup(&mut self) -> Result<String, String> {
    self.view.writeln(
      "Cleaning up connected gamepads... This will take a moment.".to_string()
    );
    self.input_map.clear();
    return self.model.cleanup();
  }

  // Exits the client, which is effectively stopping then process::exit().
  fn exit(&mut self) -> Result<String, String> {
    if self.running {
      return match self.stop() {
        Ok(_) => self.exit_ok(),
        Err(e) => self.exit_err(e)
      }
    } else {
      return self.exit_ok();
    }
  }

  // Generically exits the client as a success.
  fn exit_ok(&mut self) -> Result<String, String> {
    self.view.writeln("The client exited successfully. Goodbye!".to_string());
    process::exit(0);
  }

  // Exits the client with an associated error message.
  fn exit_err(&mut self, e: String) -> Result<String, String> {
    self.view.writeln(
      format!(
        "The client did not exit successfully. The following error occurred: \
        {}", e
      )
    );
    process::exit(1);
  }

  /**
   * Updates this controller, which then updates the model and view accordingly.
   * This is also where input events are received and parsed.
   *
   * This should be used at a fixed time interval.
   */
  pub fn update(&mut self) -> () {
    match self.view.update() {
      Ok(_) => (),
      Err(e) => {
        self.view.writeln(
          format!(
            "The following error occurred while updating the view: {}", e
          )
        );
        match self.exit() {
          Ok(_) => (),
          Err(_) => ()
        }
      }
    }
    self.parse_command_buffer();

    if self.running { 
      self.update_inputs();
      if let Err(e) = self.update_server() {
        self.view.writeln(format!("{}", e));
      }
    }
  }

  /**
   * Tells the model to update the input server. If an issue occurs while doing
   * so, the client will attempt to stop and cleanup immediately.
   */
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

  // Update everything related to inputs on this controller.
  fn update_inputs(&mut self) -> () {
    self.disconnect_inactive();
    self.fill_input_buffer();
    self.parse_input_buffer();
  } 

  // Disconnects all disconnected controllers from this controller.
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
   * Attempts to assign the given gamepad ID and switch pad type to an open
   * slot, while mapping said ID the corresponding index. Slots are open so as
   * long as they are not equal to None, or if the associated controller is
   * reported by the respective input reader as disconnected.
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

  // Parses all buffered commands within the view.
  fn parse_command_buffer(&mut self) -> () {
    while let Some(command) = self.view.get_command_buffer().pop() {
      let parts: Vec<&str> = command.split(" ").collect::<Vec<&str>>();
      match self.parse_command(&parts[0], &parts[1..]) {
        Ok(msg) => self.view.writeln(format!("{}", msg)),
        Err(e) => self.view.writeln(format!("{}", e))
      }
    }
  }

  /**
   * Either returns a list of all available commands, or provides specific usage
   * info about a given command.
   */
  fn help(&self, command: Option<&str>) -> Result<String, String> {
    return match command {
      None => Ok(
        "\n
        help (command): Provides a list of available commands. You can specify \
        a command after 'help' to view its full usage info.
        \n
        restart: Restarts the client. The client must be running.
        \n
        start: Starts the client.
        \n
        stop: Stops the client and disconnects all connected gamepads.
        \n
        exit: Same as 'stop', but totally exits the application.
        \n
        set_server_ip 'server_ip': \
        Sets the server IP to whatever 'server_ip' is. Use 'help set_server_ip \
        ' for full usage info.
        \n
        set_switch_pad 'i' 'switch_pad': \
        Sets the Switch controller type of the gamepad at slot ('i' + 1). Use \
        'help set_switch_pad' for full usage info.
        \n
        set_input_delay 'i' 'input_delay': \
        Sets the input delay of the gamepad at slot ('i' + 1). Use 'help \
        set_input_delay' for full usage info."
        .to_string()
      ),
      Some(keyword) => {
        match keyword {
          "help" => Ok(
            "\n
            Usage: help (command)
            \n
            (command) can be the name of any command.
            \n
            Example, if you want to see the usage of 'set_server_ip':
            \n
            help set_server_ip"
            .to_string()
          ),
          "restart" => Ok("\nUsage: restart".to_string()),
          "start" => Ok("\nUsage: start".to_string()),
          "stop" => Ok("\nUsage: stop".to_string()),
          "exit" => Ok("\nUsage: exit".to_string()),
          "set_server_ip" => Ok(
            "\n
            Usage: set_server_ip 'server_ip'
            \n
            Example, if your Switch's IP is 192.168.1.199:
            \n
            set_server_ip 192.168.1.199"
            .to_string()
          ),
          "set_switch_pad" => Ok(
            "\n
            Usage: set_switch_pad 'i' 'switch_pad'
            \n
            'i' must be either 0 or a positive integer. It also represents the \
            target index: slot numbers are always equal to 'i' + 1.
            \n
            switch_pad must be one of: Disconnected, ProController, \
            JoyConLSide, or JoyConRSide.
            \n
            Example, if you want to set the controller in slot 2 to a sideways \
            left JoyCon:
            \n
            set_switch_pad 1 JoyConLSide"
            .to_string()
          ),
          "set_input_delay" => Ok(
            "\n
            Usage: set_input_delay 'i' 'input_delay'
            \n
            'i' must be either 0 or a positive integer. It also represents the \
            target index: slot numbers are always equal to 'i' + 1.
            \n
            'input_delay' must be either 0 or a positive integer less than 256.
            \n
            Example, if you want to set the input delay of the controller in \
            slot 3 to 6 frames:
            \n
            set_input_delay 2 6"
            .to_string()
          ),
          _ => Err(format!("'{}' is not a valid command.", keyword))
        }
      }
    }
  }

  /**
   * Parses a given command. A command is decided by a keyword and its
   * associated arguments.
   */
  fn parse_command(
    &mut self, keyword: &str, args: &[&str]
  ) -> Result<String, String> {
    return match keyword {
      "help" => {
        if args.len() >= 1 {
          return self.help(Some(args[0]));
        } else {
          return self.help(None);
        }
      },
      "restart" => self.restart(),
      "start" => self.start(),
      "stop" => self.stop(),
      "exit" => self.exit(),
      "set_server_ip" => {
        if args.len() >= 1 {
          return self.set_server_ip(&args[0].to_string());
        } else {
          return Err(self.help(Some("set_server_ip")).unwrap());
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
        return Err(self.help(Some("set_switch_pad")).unwrap());
      },
      "set_input_delay" => {
        if args.len() >= 2 {
          if let Ok(i) = args[0].parse::<usize>() {
            if let Ok(input_delay) = args[1].parse::<u8>() {
              return self.set_input_delay(&i, &input_delay);
            }
          }
        }
        return Err(self.help(Some("set_input_delay")).unwrap());
      },
      _ => Err(format!("'{}' is not a valid command.", keyword))
    }
  }
}


