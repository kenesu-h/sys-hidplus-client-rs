use iced_wgpu::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
  layout, mouse, Background, Color, Element, Hasher, Layout, Length, Point, Rectangle, Size, Widget
};

pub struct Gamepad {
  keyout: i32,
  left: (i32, i32),
  right: (i32, i32)
}

impl Gamepad {
  pub fn new() -> Gamepad {
    return Gamepad {
      keyout: 0,
      left: (0, 0),
      right: (0, 0)
    }
  }
}

impl<Message, B> Widget<Message, Renderer<B>>
