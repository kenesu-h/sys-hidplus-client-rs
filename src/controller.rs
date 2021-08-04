use crate::{
  config::Config,
  model::ClientModel,
  input::{
    adapter::common::{
      InputButton,
      InputEvent,
      InputAdapter
    },
    switch::SwitchPad
  }
};

use std::{
  collections::{
    HashMap,
    HashSet
  }
};

pub struct InputController {
  switch_pads: Vec<SwitchPad>,
  input_delays: Vec<i8>,
  model: ClientModel,

  input_adapter: Box<dyn InputAdapter>,
  input_map: HashMap<usize, usize>,
  input_buffer: Vec<(InputEvent, i8)>
}

impl InputController {
  pub fn new(
    config: &Config, model: ClientModel, input_adapter: Box<dyn InputAdapter>
  ) -> InputController {
    return InputController {
      switch_pads: config.get_switch_pads().clone(),
      input_delays: config.get_input_delays().clone(),
      model: model,

      input_adapter: input_adapter,
      input_map: HashMap::new(),
      input_buffer: vec!()
    }
  }

  pub fn update_inputs(&mut self) -> () {
    self.disconnect_inactive();
    self.fill_buffer();
    self.parse_buffer();
  }

  pub fn update_server(&mut self) -> Result<(), String> {
    return self.model.update_server();
  }

  pub fn cleanup(&mut self) -> Result<String, String> {
    return self.model.cleanup();
  }

  fn disconnect_inactive(&mut self) -> () {
    for (gamepad_id, _) in self.input_map.clone() {
      if !self.input_adapter.is_connected(&gamepad_id) {
        match self.disconnect(&gamepad_id) {
          Ok(msg) => println!("{}", msg),
          Err(e) => println!("{}", e)
        }
      }
    }
  }

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

  /**
   * A helper method to fill the input buffer with events from the input
   * adapter.
   */
  fn fill_buffer(&mut self) -> () {
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

  /**
   * A helper method that parses events from the input buffer and updates
   * corresponding gamepads.
   */
  fn parse_buffer(&mut self) -> () {
    let mut new_buffer: Vec<(InputEvent, i8)> = vec!();
    while let Some((event, delay)) = self.input_buffer.pop() {
      if delay == 0 {
        if let Some(i) = self.input_map.get(event.get_gamepad_id()) {
          self.model.update_pad(&i, &event);
        } else {
          if let InputEvent::GamepadButton(gamepad_id, button, value) = event {
            if button == InputButton::RightBumper && value == 1.0 {
              match self.connect(&gamepad_id) {
                Ok(msg) => println!("{}", msg),
                Err(e) => println!("{}", e)
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
   * A helper method that attempts to assign the given gamepad ID and switch pad
   * type to an open slot, while mapping said ID the corresponding index. Slots
   * are open so as long as they are not equal to None, or if the associated
   * controller is reported by the respective input reader as disconnected.
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
}


