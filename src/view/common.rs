pub trait ClientView {
  fn initialize(&mut self) -> Result<(), ()>;

  /**
   * Updates this view.
   *
   * As vague as this is, you should probably expect this to do something along
   * the lines of filling the command buffer in some way.
   */
  fn update(&mut self) -> Result<(), String>;

  // Writes a given string to this view.
  fn write(&mut self, s: String) -> ();

  // Writes a given string to this view, but with a new line at the end.
  fn writeln(&mut self, s: String) -> ();

  /**
   * Returns a copy of this view's command buffer.
   *
   * The command buffer should ideally be cleared after this, so we can avoid
   * reading the same buffered commands.
   */
  fn get_command_buffer(&mut self) -> Vec<String>;
}
