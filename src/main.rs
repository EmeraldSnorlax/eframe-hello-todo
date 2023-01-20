#[derive(Default, serde::Deserialize, serde::Serialize)]
struct Task {
    title: String,
    completed: bool,
    description: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct TodoWindow {
    tasks: Vec<Task>,
}

impl Default for TodoWindow {
    fn default() -> Self {
        Self {
            tasks: vec![
                Task {
                    title: "Task 1".to_string(),
                    completed: false,
                    description: Some("This is a description".to_string()),
                },
                Task {
                    title: "Task 2".to_string(),
                    completed: true,
                    description: None,
                },
            ],
        }
    }
}

impl TodoWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for TodoWindow {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { tasks } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Todo",
        native_options,
        Box::new(|cc| Box::new(TodoWindow::new(cc))),
    );
}
