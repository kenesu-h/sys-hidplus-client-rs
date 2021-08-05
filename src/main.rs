pub mod config;
pub mod controller;
pub mod input;
pub mod model;
pub mod view;

use crate::{
  config::Config,
  input::adapter::sdl::SdlAdapter,
  model::ClientModel,
  controller::{
    ClientController
  },
  view::{
    common::ClientView,
    cli::CLIView
  }
};
use crossbeam_channel::{bounded, tick, Receiver, select};
use std::time;

#[macro_use(c)]
extern crate cute;

#[macro_use]
extern crate structure;

// A helper function to clean up the client, whether upon an error or closing.
fn main() -> Result<(), ctrlc::Error> {
  match ClientModel::new() {
    Ok(model) => {
      let mut controller: ClientController = ClientController::new(
        model,
        Box::new(CLIView::new()),
        Box::new(SdlAdapter::new())
      );
      let ticks = tick(time::Duration::from_secs_f32(1.0 / 60.0));

      match controller.initialize() {
        Ok(_) => loop {
          select! {
            recv(ticks) -> _ => {
              controller.update();
            }
          }
        },
        Err(e) => panic!("{}", e)
      }
    },
    Err(e) => panic!("{}", e)
  }
}
