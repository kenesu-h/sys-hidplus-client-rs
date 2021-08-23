pub mod config;
pub mod controller;
pub mod app;
pub mod input;
pub mod model;

use crate::app::{
  common::ClientApp,
  cli::CliApp,
  iced::application::IcedApp
};

#[macro_use(c)]
extern crate cute;

#[macro_use]
extern crate structure;

fn main() -> () {
  CliApp::initialize();
}
