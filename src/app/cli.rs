use crate::{
  app::common::{ClientApp, ClientMessage},
  input::{
    adapter::sdl::SdlAdapter,
    switch::SwitchPad
  },
  model::ClientModel,
  controller::ClientController
};

use crossbeam_channel::{tick, select};
use std::{
  io::stdin,
  str::FromStr,
  sync::{
    mpsc,
    mpsc::{Receiver, TryRecvError}
  },
  thread,
  time::Duration
};

/**
 * Represents a command line application.
 * 
 * Like all other application structs, the application must have a controller
 * to send function calls to. However, since this reads from stdin (command
 * line input), we must also have a receiver to do so.
 */
pub struct CliApp {
  controller: ClientController,
  receiver: Receiver<String>
}

/**
 * Spawns a channel that gives us a receiver, which allows us to read from
 * stdin.
 * 
 * However, this has an unfortunate side effect of reading arrow key inputs
 * too, which isn't quite supposed to happen.
 */
fn spawn_stdin_channel() -> Receiver<String> {
  let (tx, rx) = mpsc::channel::<String>();
  thread::spawn(move || loop {
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer).unwrap();
    tx.send(buffer).unwrap();
  });
  return rx;
}

impl CliApp {
  /**
   * Constructs a new command line application using a controller to read from
   * SDL, and a receiver to read from stdin.
   */
  pub fn new() -> CliApp {
    match ClientModel::new() {
      Ok(model) => return CliApp {
        controller: ClientController::new(model, Box::new(SdlAdapter::new())),
        receiver: spawn_stdin_channel()
      },
      Err(e) => panic!("{}", e)
    }
  }

  /**
   * Starts the main event loop by first initializing the controller, then
   * looping at a rate of 60 FPS. Each loop, the application will read from
   * stdin, parse it, and update the controller.
   */
  fn start_loop(&mut self) -> () {
    let ticks = tick(Duration::from_secs_f32(1.0 / 60.0));
    match self.controller.initialize() {
      Ok(o) => self.write_ok(o),
      Err(e) => panic!("{}", e)
    }
    println!("Type 'start' to begin the client or 'exit' to close it.");
    println!("Type 'help' for a list of all available commands.");

    loop {
      select! {
        recv(ticks) -> _ => { 
          match self.parse_buffer() {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
          }
          self.parse_message(ClientMessage::Tick);
        }
      }
    }
  } 

  // Parses the buffer in stdin, transforming it into a message.
  fn parse_buffer(&mut self) -> Result<(), String> {
    return match self.receiver.try_recv() {
      Ok(buffered) => {
        let parts: Vec<&str> = buffered
          .strip_suffix("\r\n")
          .or(buffered.strip_suffix("\n"))
          .unwrap_or(&buffered)
          .split(" ")
          .collect::<Vec<&str>>();
        
        match self.to_message(parts[0], &parts[1..]) {
          Ok(message) => self.parse_message(message),
          Err(e) => println!("{}", e)
        }
        return Ok(());
      },
      Err(TryRecvError::Empty) => Ok(()),
      Err(TryRecvError::Disconnected) => Err(
        String::from("stdin channel is disconnected.")
      )
    } 
  }

  /**
   * Transforms a keyword and argument combination into a message to be parsed.
   * 
   * Returns an Ok if the combination could be successfully parsed, but an Err
   * otherwise.
   */
  fn to_message(
    &mut self, keyword: &str, args: &[&str]
  ) -> Result<ClientMessage, String> {
    return match keyword {
      "start" => Ok(ClientMessage::Start),
      "stop" => Ok(ClientMessage::Stop),
      "restart" => Ok(ClientMessage::Restart),
      "exit" => Ok(ClientMessage::Exit),
      "set_server_ip" => {
        if args.len() >= 1 {
          if let Ok(server_ip) = args[0].parse::<String>() {
            return Ok(ClientMessage::SetServerIp(server_ip))
          } else {
            return Err(
              format!(
                "'{}' could not be parsed into a string.",
                args[0]
              )
            );
          }
        } else {
          return Err(
            String::from("set_server_ip requires at least one argument.")
          );
        }
      },
      "set_switch_pad" => {
        if args.len() >= 2 {
          if let Ok(i) = args[0].parse::<usize>() {
            if let Ok(switch_pad) = SwitchPad::from_str(args[1]) {
              return Ok(ClientMessage::SetSwitchPad(i, switch_pad));
            } else {
              return Err(
                format!(
                  "'{}' could not be parsed into a switch gamepad type.",
                  args[1]
                )
              );
            }
          } else {
            return Err(
              format!(
                "'{}' could not be parsed into a natural number (0 or higher).",
                args[0]
              )
            );
          }
        } else {
          return Err(
            String::from("set_switch_pad requires at least two arguments.")
          );
        }
      },
      "set_input_delay" => {
        if args.len() >= 2 {
          if let Ok(i) = args[0].parse::<usize>() {
            if let Ok(input_delay) = args[1].parse::<u8>() {
              return Ok(ClientMessage::SetInputDelay(i, input_delay));
            } else {
              return Err(
                format!(
                  "'{}' could not be parsed into an 8-bit unsigned integer [0 to 255].",
                  args[1]
                )
              );
            }
          } else {
            return Err(
              format!(
                "'{}' could not be parsed into a natural number (0 or higher).",
                args[0]
              )
            );
          }
        } else {
          return Err(
            String::from("set_input_delay requires at least two arguments.")
          );
        }
      },
      "set_left_deadzone" => {
        if args.len() >= 2 {
          if let Ok(i) = args[0].parse::<usize>() {
            if let Ok(deadzone) = args[1].parse::<f32>() {
              return Ok(ClientMessage::SetLeftDeadzone(i, deadzone));
            } else {
              return Err(
                format!(
                  "'{}' could not be parsed into a 32-bit float (a decimal).",
                  args[1]
                )
              );
            }
          } else {
            return Err(
              format!(
                "'{}' could not be parsed into a natural number (0 or higher).",
                args[0]
              )
            );
          }
        } else {
          return Err(
            String::from("set_left_deadzone requires at least two arguments.")
          );
        }
      },
      "set_right_deadzone" => {
        if args.len() >= 2 {
          if let Ok(i) = args[0].parse::<usize>() {
            if let Ok(deadzone) = args[1].parse::<f32>() {
              return Ok(ClientMessage::SetLeftDeadzone(i, deadzone));
            } else {
              return Err(
                format!(
                  "'{}' could not be parsed into a 32-bit float (a decimal).",
                  args[1]
                )
              );
            }
          } else {
            return Err(
              format!(
                "'{}' could not be parsed into a natural number (0 or higher).",
                args[0]
              )
            );
          }
        } else {
          return Err(
            String::from("set_right_deadzone requires at least two arguments.")
          );
        }
      },
      "set_anarchy_mode" => {
        if args.len() >= 1 {
          if let Ok(anarchy_mode) = args[0].parse::<bool>() {
            return Ok(ClientMessage::SetAnarchyMode(anarchy_mode));
          } else {
            return Err(
              format!(
                "'{}' could not be parsed into a bool ('true' or 'false').",
                args[0]
              )
            )
          }
        } else {
          return Err(
            String::from("set_anarchy_mode requires at least one argument.")
          );
        }
      }
      "help" => {
        if args.len() >= 1 {
          return Ok(ClientMessage::Help(Some(String::from(args[0]))));
        } else {
          return Ok(ClientMessage::Help(None));
        }
      },
      _ => Err(format!("'{}' is not a valid command.", keyword))
    }
  }

  // Parses a message into its respective method call.
  fn parse_message(&mut self, message: ClientMessage) -> () {
    match message {
      ClientMessage::Tick => self.update(),
      ClientMessage::Start => self.start(),
      ClientMessage::Stop => self.stop(),
      ClientMessage::Restart => self.restart(),
      ClientMessage::Exit => self.exit(),
      ClientMessage::Help(maybe_command) => self.help(&maybe_command),
      ClientMessage::SetServerIp(server_ip) => self.set_server_ip(&server_ip),
      ClientMessage::SetSwitchPad(i, switch_pad) => self.set_switch_pad(&i, &switch_pad),
      ClientMessage::SetInputDelay(i, input_delay) => self.set_input_delay(&i, &input_delay),
      ClientMessage::SetLeftDeadzone(i, deadzone) => self.set_left_deadzone(&i, &deadzone),
      ClientMessage::SetRightDeadzone(i, deadzone) => self.set_right_deadzone(&i, &deadzone), 
      ClientMessage::SetAnarchyMode(anarchy_mode) => self.set_anarchy_mode(&anarchy_mode),
      ClientMessage::ServerIPInputChanged(_) => (),
      ClientMessage::InputDelayInputChanged(_) => (),
      ClientMessage::LeftDeadzoneInputChanged(_) => (),
      ClientMessage::RightDeadzoneInputChanged(_) => (),
      ClientMessage::TrySetAll(_, _, _, _) => (),
      ClientMessage::ScreenChanged(_) => ()
    }
  }

  // Updates the controller and prints out all results.
  fn update(&mut self) -> () {
    for result in self.controller.update() {
      match result {
        Ok(o) => {
          if !o.is_empty() {
            self.write_ok(o);
          }
        },
        Err(e) => self.write_err(e)
      }
    }
  }

  // Tells the controller to start the client.
  fn start(&mut self) -> () {
    match self.controller.start() {
      Ok(o) => self.write_ok(o),
      Err(e) => self.write_err(e)
    }
  }

  // Tells the controller to stop the client.
  fn stop(&mut self) -> () {
    match self.controller.stop() {
      Ok(o) => self.write_ok(o),
      Err(e) => self.write_err(e)
    }
  }

  // Tells the controller to restart the client.
  fn restart(&mut self) -> () {
    match self.controller.restart() {
      Ok(o) => self.write_ok(o),
      Err(e) => self.write_err(e)
    }
  }

  /**
   * Tells the controller to exit the client or at least prepare to.
   * 
   * Exits with a code of 0 if it's okay to exit, but panics otherwise.
   */
  fn exit(&mut self) -> () {
    match self.controller.exit_prep() {
      Ok(_) => std::process::exit(0),
      Err(e) => panic!("{}", e)
    }
  }

  // Tells the controller to set the server IP.
  fn set_server_ip(&mut self, server_ip: &String) -> () {
    match self.controller.set_server_ip(server_ip) {
      Ok(o) => self.write_ok(o),
      Err(e) => self.write_err(e)
    }
  }

  // Tells the controller to set the Switch pad type of a slot.
  fn set_switch_pad(&mut self, i: &usize, switch_pad: &SwitchPad) -> () {
    match self.controller.set_switch_pad(i, switch_pad) {
      Ok(o) => self.write_ok(o),
      Err(e) => self.write_err(e)
    }
  }

  // Tells the controller to set the input delay of a slot.
  fn set_input_delay(&mut self, i: &usize, input_delay: &u8) -> () {
    match self.controller.set_input_delay(i, input_delay) {
      Ok(o) => self.write_ok(o),
      Err(e) => self.write_err(e)
    }
  }

  // Tells the controller to set the left deadzone of a slot.
  fn set_left_deadzone(&mut self, i: &usize, deadzone: &f32) -> () {
    match self.controller.set_left_deadzone(i, deadzone) {
      Ok(o) => self.write_ok(o),
      Err(e) => self.write_err(e)
    }
  }

  // Tells the controller to set the right deadzone of a slot.
  fn set_right_deadzone(&mut self, i: &usize, deadzone: &f32) -> () {
    match self.controller.set_right_deadzone(i, deadzone) {
      Ok(o) => self.write_ok(o),
      Err(e) => self.write_err(e)
    }
  }

  // Tells the controller to either enable or disable anarchy mode.
  fn set_anarchy_mode(&mut self, anarchy_mode: &bool) -> () {
    match self.controller.set_anarchy_mode(anarchy_mode) {
      Ok(o) => self.write_ok(o),
      Err(e) => self.write_err(e)
    }
  }

  /**
   * Either returns a list of all available commands, or provides specific usage
   * info about a given command.
   */
  fn help(&self, maybe_command: &Option<String>) -> () {
    match maybe_command {
      None => println!(
        "\n
        help (maybe_command): Provides a list of available commands. You can specify \
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
        set_input_delay' for full usage info.
        \n
        set_left_deadzone 'i' 'deadzone': \
        Sets the left analog stick deadzone of the gamepad at slot ('i' + 1). \
        Use 'help set_left_deadzone' for full usage info.
        \n
        set_right_deadzone 'i' 'deadzone': \
        Sets the right analog stick deadzone of the gamepad at slot ('i' + 1). \
        Use 'help set_left_deadzone' for full usage info.
        \n
        set_anarchy_mode 'anarchy_mode': \
        Sets whether anarchy mode is enabled. Use 'help set_anarchy_mode' for \
        full usage info."
      ),
      Some(keyword) => {
        match keyword.as_str() {
          "help" => println!(
            "\n
            Usage: help (command)
            \n
            (command) can be the name of any command.
            \n
            Example, if you want to see the usage of 'set_server_ip':
            \n
            help set_server_ip"
          ),
          "restart" => println!("\nUsage: restart"),
          "start" => println!("\nUsage: start"),
          "stop" => println!("\nUsage: stop"),
          "exit" => println!("\nUsage: exit"),
          "set_server_ip" => println!(
            "\n
            Usage: set_server_ip 'server_ip'
            \n
            Example, if your Switch's IP is 192.168.1.199:
            \n
            set_server_ip 192.168.1.199"
          ),
          "set_switch_pad" => println!(
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
          ),
          "set_input_delay" => println!(
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
          ),
          "set_left_deadzone" => println!(
            "\n
            Usage: set_left_deadzone 'i' 'deadzone'
            \n
            'i' must be either 0 or a positive integer. It also represents the \
            target index: slot numbers are always equal to 'i' + 1.
            \n
            'deadzone' must be a decimal number either equal to 0.0, 1.0, or \
            somewhere in between.
            \n
            Example, if you want to set the deadzone of the controller in slot 6
            to 75% of the analog stick:
            \n
            set_left_deadzone 5 0.75"
          ),
          "set_right_deadzone" => println!(
            "\n
            Usage: set_right_deadzone 'i' 'deadzone'
            \n
            'i' must be either 0 or a positive integer. It also represents the \
            target index: slot numbers are always equal to 'i' + 1.
            \n
            'deadzone' must be a decimal number either equal to 0.0, 1.0, or \
            somewhere in between.
            \n
            Example, if you want to set the deadzone of the controller in slot 6
            to 75% of the analog stick:
            \n
            set_right_deadzone 5 0.75"
          ),
          "set_anarchy_mode" => println!(
            "\n
            Usage: set_anarchy_mode 'anarchy_mode'
            \n
            'anarchy_mode' must be either 'true' or 'false'.
            \n
            Example, if you want to enable anarchy mode:
            \n
            set_anarchy_mode true"
          ),
          _ => println!("'{}' is not a valid command.", keyword)
        }
      }
    }
  }

  // Writes a string to stdout as an Ok message.
  fn write_ok(&self, s: String) -> () {
    println!("OK: {}", s);
  }

  // Writes a string to stdout as an error message.
  fn write_err(&self, s: String) -> () {
    println!("ERR: {}", s);
  }
}

impl ClientApp for CliApp {
  fn initialize() -> () {
    let mut app: CliApp = CliApp::new();
    app.start_loop();
  }
}
