use egui::TextBuffer;

#[derive(Default, serde::Deserialize, serde::Serialize)]
struct Task {
    title: String,
    completed: bool,
    description: String,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            title: "New task".to_string(),
            completed: false,
            description: "Edit description...".to_string(),
        }
    }
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
                    description: "This is a description".to_string(),
                },
                Task {
                    title: "Task 2".to_string(),
                    completed: true,
                    description: "Edit description...".to_string(),
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

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { tasks } = self;
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            // Full width button
            if ui.button("Add task").clicked() {
                tasks.push(Task::default());
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Your tasks");
            ui.separator();
            for task in tasks {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut task.completed, "");
                    ui.add(egui::TextEdit::singleline(&mut task.title));
                });
                ui.add(egui::TextEdit::multiline(&mut task.description));
                ui.separator();
            }
        });
    }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(400.0, 600.0));
    native_options.resizable = false;
    eframe::run_native(
        "Todo",
        native_options,
        Box::new(|cc| Box::new(TodoWindow::new(cc))),
    );
}
