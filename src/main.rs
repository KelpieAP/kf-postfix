use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Convertidor de Notaciones",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
struct MyApp {
    input_text: String,
    prefix_result: String,
    postfix_result: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_text: "".to_owned(),
            prefix_result: "".to_owned(),
            postfix_result: "".to_owned(),
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            
            ui.vertical(|ui| {
                ui.label("Ingresa la expresion:");
                ui.add_sized(
                    [ui.available_width() - 40.0, 30.0],
                    egui::TextEdit::singleline(&mut self.input_text)
                        .hint_text("ej. (a + b) * c"),
                );

                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Boton").clicked() {
                            self.prefix_result = format!("Prefija: {}", self.input_text);
                            self.postfix_result = format!("Postfija: {}", self.input_text);
                        }
                    });
                });

                ui.add_space(20.0);
                ui.horizontal_top(|ui| {
                    let column_width = (ui.available_width() - 20.0) / 2.0;

                    ui.allocate_ui_with_layout(
                        egui::vec2(column_width, 100.0),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            //ui.label("Result Prefija:");
                            ui.add_sized(
                                [column_width, 80.0],
                                egui::TextEdit::multiline(&mut self.prefix_result)
                                    .interactive(false),
                            );
                        },
                    );
                    
                    ui.add_space(20.0);

                    ui.allocate_ui_with_layout(
                        egui::vec2(column_width, 100.0),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                           // ui.label("Result Postfija:");
                            ui.add_sized(
                                [column_width, 80.0],
                                egui::TextEdit::multiline(&mut self.postfix_result)
                                    .interactive(false),
                            );
                        },
                    );
                });
            });
        });
    }
}