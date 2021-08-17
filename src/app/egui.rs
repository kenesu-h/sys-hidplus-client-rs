use crate::{
  app::common::ClientApp,
  input::adapter::sdl::SdlAdapter,
  model::ClientModel,
  controller::ClientController,
  view::cli::CLIView
};

use eframe::{egui, epi};

pub struct EguiApp {
  controller: ClientController,
  started: bool,
  latest: String
}

impl EguiApp {
  pub fn new() -> EguiApp {
    match ClientModel::new() {
      Ok(model) => {
        return EguiApp {
          controller: ClientController::new(
            model,
            Box::new(CLIView::new()),
            Box::new(SdlAdapter::new())
          ),
          started: false,
          latest: "".to_owned()
        }
      },
      Err(e) => panic!("{}", e)
    }
  }

  fn started_to_label(&self) -> &str {
    match &self.started {
      false => "Start",
      true => "Stop"
    }
  }
}

impl ClientApp for EguiApp {
  fn initialize() -> () {
    eframe::run_native(Box::new(Self::new()), eframe::NativeOptions::default());
  }
}

impl epi::App for EguiApp {
  fn name(&self) -> &str {
    return "client-rs";
  }

  fn setup(
    &mut self,
    _ctx: &egui::CtxRef,
    _frame: &mut epi::Frame<'_>,
    _storage: Option<&dyn epi::Storage>
  ) {
    match self.controller.initialize() {
      Ok(o) => self.latest = format!("OK: {:?}", o),
      Err(e) => self.latest = format!("ERR: {}", e)
    }
  }

  fn save(&mut self, _storage: &mut dyn epi::Storage) {
    
  }

  fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.vertical_centered(|ui| {
        ui.heading("client-rs");
        if ui.button(self.started_to_label()).clicked() {
          if !self.started {
            match self.controller.start() {
              Ok(o) => self.latest = format!("OK: {}", o),
              Err(e) => self.latest = format!("ERR: {}", e)
            }
          } else {
            match self.controller.stop() {
              Ok(o) => self.latest = format!("OK: {}", o),
              Err(e) => self.latest = format!("ERR: {}", e)
            }
          }
          self.started = !self.started;
        }
      });
    });

    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
      ui.label(&self.latest)
    });

    self.controller.update();
  }
}
