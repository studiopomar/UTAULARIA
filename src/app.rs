use eframe::egui;
use crate::voicebank::Voicebank;
use crate::io;

pub struct UtaulariaApp {
    pub voicebanks: Vec<Voicebank>,
    pub selected_index: Option<usize>,
    pub validation_report: String,
}

impl Default for UtaulariaApp {
    fn default() -> Self {
        Self {
            voicebanks: Vec::new(),
            selected_index: None,
        }
    }
}

impl UtaulariaApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let voicebanks = io::load_voicebanks();
        Self {
            voicebanks,
            selected_index: None,
            validation_report: String::new(),
        }
    }
}

impl eframe::App for UtaulariaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("UTAULARIA");
            
            ui.horizontal(|ui| {
                if ui.button("Adicionar").clicked() {
                    self.voicebanks.push(Voicebank::new());
                    self.selected_index = Some(self.voicebanks.len() - 1);
                }
                if ui.button("Salvar").clicked() {
                    let _ = io::save_voicebanks(&self.voicebanks);
                }
            });

            if ui.button("Gerar README").clicked() {
                let _ = io::update_readme(&self.voicebanks);
            }

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, vb) in self.voicebanks.iter().enumerate() {
                    let label = format!("{} ({})", vb.name, vb.creator);
                    if ui.selectable_label(self.selected_index == Some(i), label).clicked() {
                        self.selected_index = Some(i);
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(index) = self.selected_index {
                let vb = &mut self.voicebanks[index];
                ui.heading("Editar Voicebank");
                
                ui.horizontal(|ui| {
                    ui.label("Nome: ");
                    ui.text_edit_singleline(&mut vb.name);
                });

                ui.horizontal(|ui| {
                    ui.label("Criador: ");
                    ui.text_edit_singleline(&mut vb.creator);
                });

                ui.horizontal(|ui| {
                    ui.label("Tipo: ");
                    ui.text_edit_singleline(&mut vb.bank_type);
                });

                ui.horizontal(|ui| {
                    ui.label("Idioma: ");
                    ui.text_edit_singleline(&mut vb.language);
                });

                ui.horizontal(|ui| {
                    ui.label("Link: ");
                    ui.text_edit_singleline(&mut vb.download_link);
                });

                ui.label("Descrição:");
                ui.text_edit_multiline(&mut vb.description);
                
                if ui.button("Remover").clicked() {
                    self.voicebanks.remove(index);
                    self.selected_index = None;
                }

                ui.separator();
                ui.heading("Validação Técnica");
                if ui.button("Validar Pasta do Voicebank").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        match io::validate_voicebank(&path) {
                            Ok(report) => self.validation_report = report,
                            Err(e) => self.validation_report = format!("Erro: {}", e),
                        }
                    }
                }

                if !self.validation_report.is_empty() {
                    ui.label(&self.validation_report);
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Selecione um voicebank para editar.");
                });
            }
        });
    }
}
