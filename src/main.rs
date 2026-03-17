mod modules;

use crate::modules::ui;

fn main() {
    // crazy nest
    let _ =
        eframe::run_native("ToDo App",
        eframe::NativeOptions::default(),
        Box::new(|cc|
            Ok(
                Box::new(
                    ui::TodoApp::new(cc)
                    ),
                )
            )
        );
}
