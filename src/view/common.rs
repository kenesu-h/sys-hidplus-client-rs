pub trait ClientView {
  fn update(&mut self) -> Result<(), String>;

  fn write(&mut self, s: String) -> ();

  fn writeln(&mut self, s: String) -> ();

  fn get_command_buffer(&mut self) -> Vec<String>;
}
