use crate::{
  app::iced::style::Theme,
  input::switch::{
    SwitchButton,
    EmulatedPad
  }
};

use iced_wgpu::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
  layout,
  mouse,
  widget,
  Background,
  Color,
  Element,
  Hasher,
  Layout,
  Length,
  Point,
  Rectangle,
  Size,
  Widget
};
use std::str::FromStr;
use svg::node::element::{
  Circle,
  Path
};

pub struct GamepadDisplay {
  theme: Theme,
  pad: EmulatedPad
}

impl GamepadDisplay {
  pub fn new(theme: Theme, pad: EmulatedPad) -> GamepadDisplay {
    return GamepadDisplay {
      theme: theme,
      pad: pad
    }
  }

  fn is_button_pressed(
    &self, attribute: &String, value: &svg::node::Value
  ) -> bool {
    if attribute == "id" {
      if let Ok(button) = SwitchButton::from_str(&value) {
        return self.pad.is_pressed(&button);
      } else {
        return false;
      }
    } else {
      return false;
    }
  }

  fn is_button(
    &self, attribute: &String, value: &svg::node::Value, button: &SwitchButton
  ) -> bool {
    if attribute == "id" {
      if let Ok(button_from) = SwitchButton::from_str(&value) {
        return button_from == *button;
      } else {
        return false;
      }
    } else {
      return false;
    }
  }

  fn try_axis_translation(
    &self, attribute: &String, value: &svg::node::Value, path: Path, button: &SwitchButton
  ) -> Result<Path, String> {
    if self.is_button(attribute, value, button) {
      let axis: &(i16, i16);
      match button {
        SwitchButton::LST => axis = self.pad.get_left(),
        SwitchButton::RST => axis = self.pad.get_right(),
        _ => return Err(String::from("Passed a button instead of an axis."))
      }
      return Ok(
        path.set(
          "transform",
          format!(
            "translate({} {})",
            ((axis.0 as f32 / 32767.0) * 5.0) as i16,
            -((axis.1 as f32 / 32767.0) * 5.0) as i16
          )
        )
      );
    } else {
      return Err(
        String::from(
          "The (attribute, value) pair did not match the given button."
        )
      );
    }
  }
  
  fn map_svg(&self) -> svg::Document {
    let mut document: svg::Document = svg::Document::new();

    let path: &str = "./assets/pro_controller.svg";
    let mut content = String::new();

    for event in svg::open(path, &mut content).unwrap() {
      match event {
        svg::parser::Event::Tag(name, _, attributes) => {
          match name {
            "svg" => {
              for (attribute, value) in attributes {
                let cloned = (&document).clone();
                document = cloned.set(attribute, value);
              }
            },
            "circle" => {
              let mut circle: Circle = Circle::new();
              circle = circle.set("fill", "#ffffff");
              for (attribute, value) in attributes {
                circle = circle.set(&attribute, value.clone());
                if self.is_button_pressed(&attribute, &value) {
                  circle = circle.set("fill", "#ff0000");
                }
              }
              let cloned = (&document).clone();
              document = cloned.add(circle);
            },
            "path" => {
              let mut path: Path = Path::new();
              path = path.set("fill", "#ffffff");
              for (attribute, value) in attributes {
                path = path.set(&attribute, value.clone());
                if self.is_button_pressed(&attribute, &value) {
                  path = path.set("fill", "#ff0000");
                }
                if let Ok(new_path) = self.try_axis_translation(
                  &attribute, &value, path.clone(), &SwitchButton::LST
                ) {
                  path = new_path;
                }
                if let Ok(new_path) = self.try_axis_translation(
                  &attribute, &value, path.clone(), &SwitchButton::RST
                ) {
                  path = new_path;
                }
              }
              let cloned = (&document).clone();
              document = cloned.add(path);
            }
            _ => {}
          }
        },
        _ => {}
      }
    }

    return document;
  }
}

impl<Message> Widget<Message, Renderer> for GamepadDisplay {
  
  fn width(&self) -> Length {
    return Length::Shrink;
  }

  fn height(&self) -> Length {
    return Length::Shrink;
  }

  fn layout(
    &self,
    _renderer: &Renderer,
    _limits: &layout::Limits
  ) -> layout::Node {
    // Placeholder
    return layout::Node::new(Size::new(150.0, 150.0));
  }

  fn hash_layout(&self, state: &mut Hasher) {

  }

  fn draw(
    &self,
    _renderer: &mut Renderer,
    _default: &Defaults,
    layout: Layout<'_>,
    _cursor_position: Point,
    _viewport: &Rectangle
  ) -> (Primitive, mouse::Interaction) {
    let mut bytes: Vec<u8> = vec!();
    svg::write(&mut bytes, &self.map_svg());
    return (
      Primitive::Svg {
        handle: widget::svg::Handle::from_memory(bytes),
        bounds: layout.bounds(),
      },
      mouse::Interaction::default()
    );
  }
}

impl <'a, Message> Into<Element<'a, Message, Renderer>> for GamepadDisplay {
  fn into(self) -> Element<'a, Message, Renderer> {
    return Element::new(self);
  }
}

/*
impl From<Theme> for GamepadDisplay {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => light::
    }
  }
}
*/
