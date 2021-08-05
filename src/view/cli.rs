use crate::{
  view::common::{
    ClientView
  }
};

use std::{
  io::stdin,
  sync::mpsc,
  sync::mpsc::{
    Receiver,
    TryRecvError
  },
  thread
};

pub struct CLIView {
  receiver: Receiver<String>,
  command_buffer: Vec<String>
}

impl CLIView {
  pub fn new() -> CLIView {
    return CLIView {
      receiver: spawn_stdin_channel(),
      command_buffer: vec!()
    }
  }
}

// https://stackoverflow.com/a/55201400
fn spawn_stdin_channel() -> Receiver<String> {
  let (tx, rx) = mpsc::channel::<String>();
  thread::spawn(move || loop {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    tx.send(buffer).unwrap();
  });
  return rx;
}

impl ClientView for CLIView {
  fn update(&mut self) -> Result<(), String> {
    return match self.receiver.try_recv() {
      Ok(command) => {
        self.command_buffer.insert(
          0,
          command
          .strip_suffix("\r\n")
          .or(command.strip_suffix("\n"))
          .unwrap_or(&command)
          .to_string()
        );
        return Ok(());
      },
      Err(TryRecvError::Empty) => Ok(()),
      Err(TryRecvError::Disconnected) => Err(
        "stdin channel is disconnected.".to_string()
      )
    }
  }

  fn write(&mut self, s: String) -> () {
    print!("[CLIENT]: {}", s);
  }

  fn writeln(&mut self, s: String) -> () {
    println!("[CLIENT]: {}", s);
  }

  fn get_command_buffer(&mut self) -> Vec<String> {
    let cloned: Vec<String> = self.command_buffer.clone();
    self.command_buffer.clear();
    return cloned;
  }
}
