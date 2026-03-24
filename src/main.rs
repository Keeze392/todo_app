mod modules;

use crate::modules::todo_ui;

fn main() {
    let eframe_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([300.0, 150.0]),
        ..Default::default()
    };

    // crazy nest
    let _ =
        eframe::run_native("ToDo App",
        eframe_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(todo_ui::TodoApp::new(cc)))
            })
        );
}
