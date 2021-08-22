pub mod config;
pub mod controller;
pub mod app;
pub mod input;
pub mod model;
pub mod view;

use crate::{
  app::{
    common::ClientApp,
    cli::CliApp,
    iced::application::IcedApp
  },
  input::adapter::sdl::SdlAdapter,
  model::ClientModel,
  controller::{
    ClientController
  },
  view::{
    cli::CLIView
  }
};
use crossbeam_channel::{tick, select};
use std::time;

#[macro_use(c)]
extern crate cute;

#[macro_use]
extern crate structure;

fn main() -> () {
  /*
  let app: Box<dyn ClientApp> = Box::new(IcedApp::new());
  app.initialize();
  */
  CliApp::initialize();
  // IcedApp::initialize();
  /*
  match ClientModel::new() {
    Ok(model) => {
      let mut controller: ClientController = ClientController::new(
        model,
        Box::new(CLIView::new()),
        // Box::new(IcedView::new()),
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
  */
}
