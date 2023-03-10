use egui::ScrollArea;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone)]
struct Task {
    title: String,
    completed: bool,
    description: String,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            completed: false,
            description: "".to_string(),
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
        Self { tasks: vec![] }
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
        tasks.retain(|t| !t.completed);
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            if ui.button("Add task").clicked() {
                tasks.push(Task::default());
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match tasks.len() {
                0 => ui.heading("No tasks"),
                1 => ui.heading("1 task"),
                n => ui.heading(format!("{} tasks", n)),
            };

            ui.separator();
            ScrollArea::vertical().show(ui, |ui|
            for task in tasks.iter_mut() {
                ui.label("Title");
                ui.add(egui::TextEdit::singleline(&mut task.title));
                ui.label("Description");
                ui.add(egui::TextEdit::multiline(&mut task.description));
                ui.checkbox(&mut task.completed, "Completed");
                ui.separator();
            })
        });
    }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(400.0, 500.0));
    native_options.resizable = false;
    eframe::run_native(
        "Todo",
        native_options,
        Box::new(|cc| Box::new(TodoWindow::new(cc))),
    );
}
