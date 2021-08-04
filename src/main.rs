pub mod config;
pub mod controller;
pub mod input;
pub mod model;

use crate::{
  config::Config,
  input::adapter::sdl::SdlAdapter,
  model::ClientModel,
  controller::{
    InputController
  },
};
use crossbeam_channel::{bounded, tick, Receiver, select};
use std::{
  cell::RefCell,
  rc::Rc,
  time
};

#[macro_use(c)]
extern crate cute;

#[macro_use]
extern crate structure;

// A helper function that opens a channel to receive Ctrl-C signals.
fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
  let (sender, receiver) = bounded(100);
  ctrlc::set_handler(move || {
      let _ = sender.send(());
  })?;

  return Ok(receiver);
}

// A helper function to clean up the client, whether upon an error or closing.
fn main() -> Result<(), ctrlc::Error> {
  match ClientModel::new() {
    Ok(mut model) => {
      let mut controller: InputController = InputController::new(
        model,
        Box::new(SdlAdapter::new())
      );
      let ticks = tick(time::Duration::from_secs_f32(1.0 / 60.0));

      controller.load_config();
      loop {
        select! {

        }
      }
    },
    Err(e) => panic!("{}", e)
  }
  // TODO: Clean up more later, eventually we'd want to move to a MVC format.
  match confy::load_path("./config.toml") {
    Ok(config) => {
      match ClientModel::new(&config) {
        Ok(mut model) => {
          /* 
           * Everything below here is pretty much thanks to the following link:
           * https://rust-cli.github.io/book/in-depth/signals.html
           */
          let mut controller: InputController = InputController::new(&config, model, Box::new(SdlAdapter::new()));
          let ctrl_c_events = ctrl_channel()?;
          let ticks = tick(time::Duration::from_secs_f32(1.0 / 60.0));
          println!("Client is ready to connect controllers.");

          loop {
            select! {
              recv(ticks) -> _ => {
                controller.update_inputs();
                if let Err(e) = controller.update_server() {
                  println!("An error occurred while attempting to update the
                    input server:");
                  println!("{}", e);
                  match controller.cleanup() {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => println!("{}", e)
                  }
                  return Ok(());
                }
              }
              recv(ctrl_c_events) -> _ => {
                match controller.cleanup() {
                  Ok(msg) => println!("{}", msg),
                  Err(e) => println!("{}", e)
                }
                return Ok(());
              }
            }
          }     
        },
        Err(e) => {
          println!("{}", e);
          return Ok(());
        }
      }
    },
    Err(e) => panic!("{}", e)
  }
}
