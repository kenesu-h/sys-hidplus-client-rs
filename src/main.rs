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

use std::env;

#[macro_use(c)]
extern crate cute;

#[macro_use]
extern crate structure;

fn main() -> () {
  let args: Vec<String> = env::args().collect();
 
  if args.contains(&String::from("cli")) {
    CliApp::initialize();
  } else {
    IcedApp::initialize();
  }
}
