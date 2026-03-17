use eframe::egui;
use egui_dnd::dnd;
//use egui::AtomExt;

#[derive(PartialEq, Hash)]
struct TodoAppWindowData {
    text: String,
    id: usize
}

pub  struct TodoApp {
    ui_item_vec: Vec<TodoAppWindowData>,
    input_text: String
}

impl TodoApp {
    // from eframe for create window, so we can design with egui
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { ui_item_vec: Vec::new(),
        input_text: String::new()
        }
    }
}

// design GUI main page
impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let vec_length: usize = self.ui_item_vec.len();

        // left side panel
        egui::SidePanel::left("ToDo_left_side").resizable(false).show(ctx, |ui_left| {
            ui_left.label("test");
            /*if ui_left.add(egui::Button::image(
                    egui::Image::new(egui::include_image!("settings.png").atom_max_width(50.0))
                    )).clicked() {

            }*/
        });

        // right side panel
        egui::CentralPanel::default().show(&ctx, |ui| {
             ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), 25.0),
                egui::Layout::right_to_left(egui::Align::RIGHT),
                |ui_right| {
                    ui_right.horizontal(|ui_horizontal| {
                        if ui_horizontal.add_sized([80.0, 30.0], egui::Button::new("Delete the first")).clicked() {
                            // check for safety, make sure it's not empty
                            if vec_length > 0 {
                                self.ui_item_vec.remove(0);
                            }
                        }

                        // get input box and check if exist in vec, add number otherwise skip loop
                        // support shortcut Key, "Enter" key
                        if ui_horizontal.add_sized([80.0, 30.0], egui::Button::new("Add")).clicked() ||
                        ui_horizontal.input(|input| input.key_pressed(egui::Key::Enter)) {
                            let mut extra_number: usize = 0;

                            // TODO: fix this logic isn't working correctly
                            if !self.ui_item_vec.iter().any(|item| item.id != extra_number) {
                                self.ui_item_vec.push(
                                    TodoAppWindowData {
                                        text: self.input_text.clone(),
                                        id: extra_number
                                    }
                                );
                                self.input_text = String::from("");

                            } else {
                                loop {
                                    extra_number += 1;

                                    if !self.ui_item_vec.iter().any(|item| item.id != extra_number) {
                                        self.ui_item_vec.push(
                                            TodoAppWindowData {
                                                text: self.input_text.clone(),
                                                id: extra_number
                                            }
                                        );

                                        self.input_text = String::from("");
                                        break;
                                    }
                                }
                            }
                        }

                        let _response_textedit = ui_horizontal.text_edit_singleline(&mut self.input_text);
                    });
            });

            // drag and drop function
            let _dnd_response: egui_dnd::DragDropResponse = 
                dnd(ui, "dnd_0").show_vec(&mut self.ui_item_vec, |ui, item, handle, _state| {
                    ui.horizontal(|ui| {

                        //ui.add();
                        // use frame?
                        egui::Frame::NONE
                            .inner_margin(8)
                            .outer_margin(12)
                            .corner_radius(8)
                            .shadow(egui::Shadow {
                                offset: [4, 10],
                                blur: 16,
                                spread: 4,
                                color: egui::Color32::from_black_alpha(180)
                            })
                            .fill(egui::Color32::GRAY)
                            .show(ui, |ui| {
                                ui.push_id(item.id, |ui| ui.add(egui::Label::new(&item.id.to_string())));
                            });

                        handle.ui(ui, |ui| { ui.label("DRAG_ME");

                        });
                    });
            });
       });
    }
}
