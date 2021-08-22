use crate::{
  app::common::{ClientApp, ClientMessage},
  input::adapter::sdl::SdlAdapter,
  model::ClientModel,
  controller::ClientController
};

use crossbeam_channel::{tick, select};
use crossterm::{
  event::{self, Event, KeyCode, KeyEvent, EventStream},
  terminal::{disable_raw_mode, enable_raw_mode}
};
use futures_timer::Delay;
use std::{
  sync::{
    mpsc,
    mpsc::{Receiver, TryRecvError}
  },
  thread,
  time::Duration
};

pub struct CliApp {
  controller: ClientController,
  receiver: Receiver<String>
}

fn spawn_stdin_channel() -> Receiver<String> {
  let (tx, rx) = mpsc::channel::<String>();
  thread::spawn(move || loop {
    tx.send(read_line().unwrap()).unwrap();
  });
  return rx;
}

// https://github.com/crossterm-rs/crossterm/blob/master/examples/event-read-char-line.rs
fn read_line() -> Result<String, std::io::Error> {
  let mut line = String::new();
  while let Event::Key(KeyEvent { code, .. }) = event::read()? {
      match code {
          KeyCode::Enter => {
              break;
          }
          KeyCode::Char(c) => {
              line.push(c);
          }
          _ => {}
      }
  }
  return Ok(line);
}

impl CliApp {
  pub fn new() -> CliApp {
    match ClientModel::new() {
      Ok(model) => return CliApp {
        controller: ClientController::new(model, Box::new(SdlAdapter::new())),
        receiver: spawn_stdin_channel()
      },
      Err(e) => panic!("{}", e)
    }
  }

  fn start_loop(&mut self) -> () {
    let ticks = tick(Duration::from_secs_f32(1.0 / 60.0));
    match self.controller.initialize() {
      Ok(o) => println!("{}", o),
      Err(e) => self.panic(e)
    }
    enable_raw_mode();

    loop {
      select! {
        recv(ticks) -> _ => { 
          match self.parse_buffer() {
            Ok(_) => (),
            Err(e) => self.panic(e)
          }
          self.parse_message(ClientMessage::Tick);
        }
      }
    }
  } 

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

  fn to_message(
    &mut self, keyword: &str, args: &[&str]
  ) -> Result<ClientMessage, String> {
    return match keyword {
      "start" => Ok(ClientMessage::Start),
      "stop" => Ok(ClientMessage::Stop),
      "exit" => Ok(ClientMessage::Exit),
      "help" => {
        if args.len() >= 1 {
          return Ok(ClientMessage::Help(String::from(args[0])));
        } else {
          return Err(String::from(""));
        }
      },
      _ => Err(format!("'{}' is not a valid command.", keyword))
    }
  }

  fn parse_message(&mut self, message: ClientMessage) -> () {
    match message {
      ClientMessage::Tick => self.update(),
      ClientMessage::Start => self.start(),
      ClientMessage::Stop => self.stop(),
      ClientMessage::Exit => self.exit(),
      _ => ()
    }
  }

  fn update(&mut self) -> () {
    match self.controller.update() {
      Ok(_) => (),
      Err(e) => self.write_err(e)
    }
  }

  fn start(&mut self) -> () {
    match self.controller.start() {
      Ok(o) => self.write_ok(o),
      Err(e) => self.write_err(e)
    }
  }

  fn stop(&mut self) -> () {
    match self.controller.stop() {
      Ok(o) => self.write_ok(o),
      Err(e) => self.write_err(e)
    }
  }

  fn exit(&mut self) -> () {
    match self.controller.exit_prep() {
      Ok(_) => {
        disable_raw_mode();
        std::process::exit(0);
      },
      Err(e) => self.panic(e)
    }
  }

  fn panic(&self, e: String) -> () {
    disable_raw_mode();
    panic!("{}", e);
  }

  fn write_ok(&self, s: String) -> () {
    println!("OK: {}", s);
  }

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
