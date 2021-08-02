pub mod client;
pub mod config;
pub mod input;

use crate::{
  input::adapter::sdl::SdlAdapter,
  client::Client, 
};
use crossbeam_channel::{bounded, tick, Receiver, select};
use std::time;

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
  // TODO: Clean up more later, eventually we'd want to move to a MVC format.
  match confy::load_path("./config.toml") {
    Ok(config) => match Client::new(config, Box::new(SdlAdapter::new())) {
      Ok(mut client) => {
        /* 
         * Everything below here is pretty much thanks to the following link:
         * https://rust-cli.github.io/book/in-depth/signals.html
         */
        let ctrl_c_events = ctrl_channel()?;
        let ticks = tick(time::Duration::from_secs_f32(1.0 / 60.0));
        println!("Client is ready to connect controllers.");

        loop {
          select! {
            recv(ticks) -> _ => {
              client.update_pads();
              if let Err(e) = client.update_server() {
                println!("An error occurred while attempting to update the
                  input server:");
                println!("{}", e);
                match client.cleanup() {
                  Ok(msg) => println!("{}", msg),
                  Err(e) => println!("{}", e)
                }
                return Ok(());
              }
            }
            recv(ctrl_c_events) -> _ => {
              match client.cleanup() {
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
    },
    Err(e) => panic!("{}", e)
  }
}
