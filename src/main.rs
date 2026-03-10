use eframe::egui;
use std::collections::HashMap;

use kf_compiler::{
    Token,
    lex_program
};

mod postfija; 
mod prefija; 

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0]), 
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
    prefija_result: String, 
    postfix_result: String,
    eval_result: String,
    prefija_result_vec: Vec<Token>, 
    postfix_result_vec: Vec<Token>,
    var_inputs: HashMap<String, String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_text: "".to_owned(),
            prefija_result: "".to_owned(), 
            postfix_result: "".to_owned(),
            eval_result: "".to_owned(),
            prefija_result_vec: Vec::new(), 
            postfix_result_vec: Vec::new(),
            var_inputs: HashMap::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            
            ui.vertical(|ui| {
                ui.label("Ingresa la expresión infija:");
                ui.add_sized(
                    [ui.available_width() - 40.0, 30.0],
                    egui::TextEdit::singleline(&mut self.input_text)
                        .hint_text("(A + B) * C"),
                );

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                       
                        if ui.button("Convertir").clicked() {
                           
                            self.postfix_result_vec = postfija::infija_a_postfija(&self.input_text);
                            self.postfix_result = postfija::token_vec_to_string(&self.postfix_result_vec);

                            self.prefija_result_vec = prefija::infija_a_prefija(&self.input_text);
                            self.prefija_result = prefija::token_vec_to_string(&self.prefija_result_vec);
                        }
                    });
                });

                ui.add_space(20.0);

                ui.label("Prefija:");
                ui.add_sized(
                    [ui.available_width() - 40.0, 30.0],
                    egui::TextEdit::singleline(&mut self.prefija_result)
                        .interactive(true),
                );

                ui.add_space(20.0);

                ui.label("Postfija:");
                ui.add_sized(
                    [ui.available_width() - 40.0, 30.0],
                    egui::TextEdit::singleline(&mut self.postfix_result)
                        .interactive(true),
                );

                ui.add_space(20.0);
  

                let ids = postfija::get_identifiers(&self.postfix_result_vec);
              
                self.var_inputs.retain(|k, _| ids.contains(k));

                for id in &ids {
                    self.var_inputs.entry(id.clone()).or_insert_with(String::new);
                }

                if !ids.is_empty() {
                    ui.add_space(10.0);
                    ui.label("Variables:");
                    for id in &ids {
                        ui.horizontal(|ui| {
                            ui.label(format!("{} = ", id));
                            if let Some(input) = self.var_inputs.get_mut(id) {
                                ui.add_sized([120.0, 20.0], egui::TextEdit::singleline(input));
                            }
                        });
                    }
                }


                ui.add_space(20.0);
             

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Evaluar").clicked() {
                                self.postfix_result_vec = lex_program(&self.postfix_result)
                                    .into_iter()
                                    .map(|t| t.token)
                                    .collect();

                            let mut vars = HashMap::new();
                            let mut all_ok = true;
                            for (name, text) in &self.var_inputs {
                                match text.trim().parse::<f32>() {
                                    Ok(val) => { vars.insert(name.clone(), val);}
                                    Err(_) => {
                                        self.eval_result = format!("Error: '{}' no es número válido", name);
                                        all_ok = false;
                                        break;
                                    }
                                }
                            }

                            if all_ok {
                                match postfija::eval_postfix(&self.postfix_result_vec, &vars) {
                                    Ok(val) => { self.eval_result = val.to_string(); }
                                    Err(e) => { self.eval_result = format!("Error: {}", e); }
                                }
                            }
                        }
                    });
                });

                ui.add_space(20.0);

                ui.label("Resultado:");
                ui.add_sized(
                    [ui.available_width() - 40.0, 30.0],
                    egui::TextEdit::singleline(&mut self.eval_result)
                        .interactive(false),
                );
            });
        });
    }
}