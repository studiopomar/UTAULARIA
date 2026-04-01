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
            validation_report: String::new(),
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
        egui_extras::install_image_loaders(ctx);

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(220.0)
            .show(ctx, |ui| {
            ui.heading("UTAULARIA");
            
            ui.horizontal(|ui| {
                if ui.button("Adicionar").clicked() {
                    let mut new_vb = Voicebank::new();
                    new_vb.folder_name = format!("Novo_VB_{}", self.voicebanks.len());
                    self.voicebanks.push(new_vb);
                    self.selected_index = Some(self.voicebanks.len() - 1);
                }
                if ui.button("Salvar Tudo").clicked() {
                    let _ = io::save_voicebanks(&self.voicebanks);
                }
            });

            ui.add_space(10.0);
            ui.label(egui::RichText::new("Ações de Repositório").strong());
            ui.separator();

            if ui.button("Sincronizar (Pull)").clicked() {
                match io::sync_with_github() {
                    Ok(msg) => self.validation_report = msg,
                    Err(e) => self.validation_report = format!("Erro: {}", e),
                }
                // Recarrega os voicebanks após o pull
                self.voicebanks = io::load_voicebanks();
            }

            if ui.button("Gerar README").clicked() {
                let _ = io::update_readme(&self.voicebanks);
                self.validation_report = "README.md atualizado com sucesso!".to_string();
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
                
                ui.vertical_centered(|ui| {
                    ui.heading(format!("Editar: {}", vb.name));
                });
                
                ui.add_space(20.0);

                ui.columns(2, |columns| {
                    // Coluna da Esquerda: Metadados
                    columns[0].vertical(|ui| {
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.label(egui::RichText::new("Informações Gerais").strong());
                            ui.separator();
                            
                            ui.horizontal(|ui| {
                                ui.label("Nome: ");
                                ui.text_edit_singleline(&mut vb.name);
                            });

                            ui.horizontal(|ui| {
                                ui.label("Pasta: ");
                                ui.text_edit_singleline(&mut vb.folder_name);
                            });

                            ui.horizontal(|ui| {
                                ui.label("Criador: ");
                                ui.text_edit_singleline(&mut vb.creator);
                            });

                            ui.horizontal(|ui| {
                                ui.label("Método: ");
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
                        });

                        ui.add_space(10.0);

                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.label(egui::RichText::new("Descrição").strong());
                            ui.separator();
                            ui.text_edit_multiline(&mut vb.description);
                        });
                    });

                    // Coluna da Direita: Preview Visual e Design
                    columns[1].vertical(|ui| {
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.label(egui::RichText::new("Design Visual").strong());
                            ui.separator();
                            
                            ui.horizontal(|ui| {
                                ui.label("Caminho:");
                                ui.text_edit_singleline(&mut vb.image_path);
                            });
                            
                            ui.add_space(10.0);
                            
                            // Preview da Imagem
                            if !vb.image_path.is_empty() {
                                let uri = if vb.image_path.starts_with("http") {
                                    vb.image_path.clone()
                                } else {
                                    // Assumindo que o caminho é relativo ao projeto ou absoluto
                                    format!("file://{}", std::path::Path::new(&vb.image_path).canonicalize().map(|p| p.to_string_lossy().to_string()).unwrap_or(vb.image_path.clone()))
                                };

                                ui.image(uri)
                                    .max_width(200.0)
                                    .rounding(10.0);
                            } else {
                                ui.centered_and_justified(|ui| {
                                    ui.label("Sem imagem de design.");
                                });
                            }
                        });
                    });
                });

                ui.add_space(20.0);
                
                ui.horizontal(|ui| {
                    if ui.button(egui::RichText::new("Remover Voicebank").color(egui::Color32::LIGHT_RED)).clicked() {
                        self.voicebanks.remove(index);
                        self.selected_index = None;
                    }
                });

                ui.separator();
                ui.heading("Validação Técnica");
                if ui.button("Validar Pasta Local").clicked() {
                    let folder_path = std::path::Path::new("voicebank_assets").join(&vb.folder_name);
                    match io::validate_voicebank(&folder_path) {
                        Ok(report) => self.validation_report = report,
                        Err(e) => self.validation_report = format!("Erro: {}", e),
                    }
                }

                if !self.validation_report.is_empty() {
                    ui.label(&self.validation_report);
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Selecione um voicebank na lista lateral para editar.");
                });
            }
        });
    }
}
