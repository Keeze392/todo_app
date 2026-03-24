use eframe::egui;
use egui_dnd::dnd;
use egui::{Color32, RichText, include_image};

#[derive(PartialEq, Hash)]
struct TodoAppWindowData {
    text: String,
    id: usize,
}

pub  struct TodoApp {
    ui_item_vec: Vec<TodoAppWindowData>,
    input_text: String,
    theme_select: egui::Theme,
}

// GUI design
impl TodoApp {
    // from eframe for create window, so we can design with egui
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { ui_item_vec: Vec::new(),
        input_text: String::new(),
        theme_select: egui::Theme::Dark,
        }
    }

    fn left_side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("ToDo_left_side")
            .min_width(125.0)
            .resizable(false)
            .show(ctx, |_ui| {

                egui::TopBottomPanel::bottom("ToDo_left_bottom_side")
                    .min_height(45.0)
                    .show_separator_line(false)
                    .show(&ctx, |ui| {

                    ui.label("Theme:");

                    egui::ComboBox::from_id_salt("ThemeMode")
                        .selected_text(format!("{:?} Mode", self.theme_select))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.theme_select, egui::Theme::Dark, "Dark Mode");
                            ui.selectable_value(&mut self.theme_select, egui::Theme::Light, "Light Mode");
                        });
                });
            });
    }

    // place in right side panel / top panel
    fn horizontal_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {

            if ui.add_sized([80.0, 30.0], egui::Button::new("Add")).clicked() ||
                ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                self.add_item();
            }

            let singline_response = ui.add(egui::TextEdit::singleline(&mut self.input_text)
                .background_color(match self.theme_select {
                    egui::Theme::Dark => egui::Color32::WHITE,
                    egui::Theme::Light => egui::Color32::DARK_GRAY,
                })
                .text_color(match self.theme_select {
                    egui::Theme::Dark => egui::Color32::BLACK,
                    egui::Theme::Light => egui::Color32::WHITE,
                }));
            
            // since singleline will lost focus after press Enter key, so we add logic to bring
            // focus back. Why is there no option to disable? lame.
            if singline_response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                singline_response.request_focus();
            }
            
        });
    }

    // inside central panel
    // so much nests...
    fn drag_and_drop(&mut self, ui: &mut egui::Ui) {
        let mut id_should_delete: Option<usize> = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            dnd(ui, "dnd_0").show_vec(&mut self.ui_item_vec, |ui, item, handle, _state| {
                ui.horizontal_top(|ui| {
                
                    TodoApp::dnd_draw_frame(
                        ui,
                        &self.theme_select,
                        handle,
                        &mut id_should_delete,
                        item
                    );

                });
            });
        });

        self.check_remove_vector_id(id_should_delete);
    }
}

// function helper call (also im not sure what perfect to call)
// make clean to read from separate impl
impl TodoApp {
    fn add_item(&mut self) {
        let mut create_id: usize = 0;

        while self.ui_item_vec.iter().any(|item| item.id == create_id) {
            create_id += 1;
        }

        self.ui_item_vec.push(TodoAppWindowData {
            text: self.input_text.clone(),
            id: create_id 
        });

        self.input_text.clear();
    }

    // check if id_should_delete is not None then remove element in vector by id
    fn check_remove_vector_id(&mut self, id_should_delete: Option<usize>) {
        if let Some(id) = id_should_delete {
            if let Some(pos) = self.ui_item_vec.iter().position(|item| item.id == id) {
                self.ui_item_vec.remove(pos);
            }
        }
    }
}

// for drag and drop function
// into small to avoid nest and clean to read
impl TodoApp {
    fn dnd_draw_label(
        ui: &mut egui::Ui,
        theme: &egui::Theme,
        text: &String,
        id: &usize) {

        ui.push_id(
            id, |ui| {
                ui.add_sized(
                    [ui.available_width() - 125.0, 0.0],
                    egui::Label::new(
                    RichText::new(text)
                        .color(match theme {
                            egui::Theme::Dark => Color32::WHITE,
                            egui::Theme::Light => Color32::BLACK,
                        })
                )
                .wrap());
        });
    }

    // a button delete and a drag icon
    fn dnd_draw_button_drag(
        ui: &mut egui::Ui,
        handle: egui_dnd::Handle,
        id: &usize,
        id_should_delete: &mut Option<usize>) {

        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 0.0),
            egui::Layout::right_to_left(egui::Align::RIGHT),
            |ui| {
                // random ghost widget required for whatever reason that drag icon won't show
                // unless any widget first before image, it shows. How strange.
                ui.add(egui::Label::new("").selectable(false));

                // icon for drag
                handle.ui(ui, |ui| { 
                    ui.add(
                        egui::Image::new(include_image!("../../assets/images/pngkey.com-hamburger-png-303853.png"))
                        .max_width(25.0));
                });

                // button for remove
                if ui.button("delete").clicked() {
                    *id_should_delete = Some(*id);
                };
        });
    }

    // draw frame each from list with label, button and drag
    fn dnd_draw_frame(
        ui: &mut egui::Ui,
        theme: &egui::Theme,
        handle: egui_dnd::Handle,
        id_should_delete: &mut Option<usize>,
        item: &mut TodoAppWindowData) {

        egui::Frame::NONE
            .inner_margin(8)
            .outer_margin(8)
            .corner_radius(8)
            .shadow(egui::Shadow {
                offset: [4, 8],
                blur: 16,
                spread: 4,
                color: Color32::from_black_alpha(180)
            })
            .fill(
                match theme {
                    egui::Theme::Dark => Color32::DARK_GRAY,
                    egui::Theme::Light => Color32::LIGHT_GRAY,
                })
            .show(ui, |ui| {

                // Label with wrap
                TodoApp::dnd_draw_label(ui, &theme, &item.text, &item.id);

                // button and drag icon
                TodoApp::dnd_draw_button_drag(ui, handle, &item.id, id_should_delete);

            });
        
    }
}

// design GUI main page
impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_theme(self.theme_select);
        self.left_side_panel(ctx);

        // right side panel
        egui::CentralPanel::default().show(&ctx, |ui| {

            ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), 25.0),
                egui::Layout::right_to_left(egui::Align::RIGHT),
                |ui_right| {
                    self.horizontal_panel(ui_right);
            });

            // drag and drop function
            self.drag_and_drop(ui);
        });
    }
}


