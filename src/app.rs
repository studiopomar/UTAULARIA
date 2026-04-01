use eframe::egui;
use crate::voicebank::Voicebank;
use crate::io;

pub enum ViewMode {
    Gallery,
    Details(usize),
}

pub struct UtaulariaApp {
    pub voicebanks: Vec<Voicebank>,
    pub selected_index: Option<usize>, // Keeping this for backward compatibility and internal use
    pub view_mode: ViewMode,
    pub search_query: String,
    pub validation_report: String,
}

impl Default for UtaulariaApp {
    fn default() -> Self {
        Self {
            voicebanks: Vec::new(),
            selected_index: None,
            view_mode: ViewMode::Gallery,
            search_query: String::new(),
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
            view_mode: ViewMode::Gallery,
            search_query: String::new(),
            validation_report: String::new(),
        }
    }
}

impl eframe::App for UtaulariaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        // Sidebar Minimalista para Ações de Admin/Sistema
        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.heading("UTAULARIA");
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("Galeria Brasileira").small().color(egui::Color32::GRAY));
                    ui.add_space(20.0);
                });

                ui.group(|ui| {
                    ui.label("Gerenciamento");
                    ui.separator();
                    if ui.button("➕ Adicionar Novo").clicked() {
                        let mut new_vb = Voicebank::new();
                        new_vb.folder_name = format!("Novo_VB_{}", self.voicebanks.len());
                        self.voicebanks.push(new_vb);
                        let idx = self.voicebanks.len() - 1;
                        self.view_mode = ViewMode::Details(idx);
                    }
                    if ui.button("💾 Salvar Tudo").clicked() {
                        let _ = io::save_voicebanks(&self.voicebanks);
                    }
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Repositório");
                    ui.separator();
                    if ui.button("🔄 Sincronizar (Pull)").clicked() {
                        match io::sync_with_github() {
                            Ok(msg) => self.validation_report = msg,
                            Err(e) => self.validation_report = format!("Erro: {}", e),
                        }
                        self.voicebanks = io::load_voicebanks();
                    }
                    if ui.button("📝 Gerar README").clicked() {
                        let _ = io::update_readme(&self.voicebanks);
                        self.validation_report = "README.md atualizado!".to_string();
                    }
                });

                if !self.validation_report.is_empty() {
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new(&self.validation_report).small().color(egui::Color32::LIGHT_BLUE));
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    ui.label("v0.2.0 - Studio POMAR");
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(ctx.style().visuals.panel_fill)) // Estilo limpo
            .show(ctx, |ui| {
                match self.view_mode {
                    ViewMode::Gallery => {
                        self.render_gallery(ui);
                    }
                    ViewMode::Details(index) => {
                        self.render_details(ui, index);
                    }
                }
            });
    }
}

impl UtaulariaApp {
    fn render_gallery(&mut self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        
        // Barra de Busca "Play Store Style"
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            let search_box = egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Pesquisar voicebanks...")
                .margin(egui::Margin::symmetric(15.0, 10.0));
            
            egui::Frame::none()
                .fill(ui.visuals().widgets.active.bg_fill)
                .corner_radius(20.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width() - 40.0);
                    ui.add(search_box);
                });
        });

        ui.add_space(20.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(25.0, 25.0);
                ui.add_space(10.0);

                let filtered_vbs: Vec<(usize, &Voicebank)> = self.voicebanks.iter().enumerate()
                    .filter(|(_, vb)| {
                        self.search_query.is_empty() || 
                        vb.name.to_lowercase().contains(&self.search_query.to_lowercase()) ||
                        vb.creator.to_lowercase().contains(&self.search_query.to_lowercase())
                    })
                    .collect();

                for (idx, vb) in filtered_vbs {
                    self.render_voicebank_card(ui, idx, vb);
                }
            });
        });
    }

    fn render_voicebank_card(&mut self, ui: &mut egui::Ui, index: usize, vb: &Voicebank) {
        let card_width = 160.0;
        let card_height = 240.0;

        let response = egui::Frame::none()
            .corner_radius(15.0)
            .fill(ui.visuals().widgets.noninteractive.bg_fill)
            .show(ui, |ui| {
                ui.set_width(card_width);
                ui.set_height(card_height);
                
                ui.vertical(|ui| {
                    // Imagem (Capa do Voicebank)
                    let img_rect = ui.available_rect_before_wrap();
                    let img_size = egui::vec2(card_width, 160.0);
                    
                    if !vb.image_path.is_empty() {
                        let uri = if vb.image_path.starts_with("http") {
                            vb.image_path.clone()
                        } else {
                            format!("file://{}", std::path::Path::new(&vb.image_path).to_string_lossy())
                        };
                        
                        ui.add(egui::Image::new(uri)
                            .fit_to_exact_size(img_size)
                            .corner_radius(egui::CornerRadius::same(15)));
                    } else {
                        // Placeholder se não houver imagem
                        let (rect, _) = ui.allocate_at_least(img_size, egui::Sense::hover());
                        ui.painter().rect_filled(rect, 15.0, egui::Color32::from_gray(40));
                        ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "No Image", egui::FontId::proportional(12.0), egui::Color32::GRAY);
                    }

                    ui.add_space(8.0);
                    
                    // Texto do Card
                    ui.vertical(|ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                           ui.label(egui::RichText::new(&vb.name).strong().size(14.0));
                           ui.label(egui::RichText::new(&vb.creator).small().color(egui::Color32::GRAY));
                        });
                    });
                });
            }).response;

        let response = ui.interact(response.rect, ui.id().with(index), egui::Sense::click());
        
        if response.hovered() {
            ui.painter().rect_stroke(response.rect, 15.0, (2.0, egui::Color32::from_rgb(0, 150, 255)));
            ctx_if_hovered_pointer_cursor(ui);
        }

        if response.clicked() {
            self.view_mode = ViewMode::Details(index);
        }
    }

    fn render_details(&mut self, ui: &mut egui::Ui, index: usize) {
        let mut should_remove = false;
        let mut go_back = false;

        ui.add_space(10.0);
        
        // Header de Detalhes
        ui.horizontal(|ui| {
            if ui.button("⬅ Voltar").clicked() {
                go_back = true;
            }
        });

        ui.add_space(20.0);

        if let Some(vb) = self.voicebanks.get_mut(index) {
            ui.horizontal(|ui| {
                // Ícone Grande
                let img_size = egui::vec2(120.0, 120.0);
                if !vb.image_path.is_empty() {
                    let uri = if vb.image_path.starts_with("http") {
                        vb.image_path.clone()
                    } else {
                        format!("file://{}", std::path::Path::new(&vb.image_path).to_string_lossy())
                    };
                    ui.add(egui::Image::new(uri).fit_to_exact_size(img_size).corner_radius(20.0));
                } else {
                    let (rect, _) = ui.allocate_at_least(img_size, egui::Sense::hover());
                    ui.painter().rect_filled(rect, 20.0, egui::Color32::from_gray(40));
                }

                ui.add_space(15.0);

                // Info da Direita
                ui.vertical(|ui| {
                    ui.heading(egui::RichText::new(&vb.name).size(28.0).strong());
                    ui.label(egui::RichText::new(&vb.creator).color(egui::Color32::from_rgb(0, 150, 255)).size(16.0));
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new(egui::RichText::new("Baixar").color(egui::Color32::WHITE).strong())
                            .fill(egui::Color32::from_rgb(0, 128, 255))
                            .min_size(egui::vec2(120.0, 35.0))
                            .corner_radius(10.0)).clicked() {
                            // Abrir link no navegador
                            let _ = webbrowser::open(&vb.download_link);
                        }
                        
                        if ui.add(egui::Button::new("Validar Técnica")
                            .min_size(egui::vec2(120.0, 35.0))
                            .corner_radius(10.0)).clicked() {
                                let folder_path = std::path::Path::new("voicebank_assets").join(&vb.folder_name);
                                match io::validate_voicebank(&folder_path) {
                                    Ok(report) => self.validation_report = report,
                                    Err(e) => self.validation_report = format!("Erro: {}", e),
                                }
                        }
                    });
                });
            });

            ui.add_space(30.0);
            ui.separator();
            ui.add_space(10.0);

            ui.columns(2, |columns| {
                columns[0].vertical(|ui| {
                    ui.label(egui::RichText::new("Sobre este Voicebank").strong().size(18.0));
                    ui.add_space(5.0);
                    ui.text_edit_multiline(&mut vb.description);
                });

                columns[1].vertical(|ui| {
                   ui.label(egui::RichText::new("Informações Técnicas").strong().size(18.0));
                   ui.add_space(5.0);
                   
                   egui::Grid::new("tech_info").num_columns(2).spacing([10.0, 10.0]).show(ui, |ui| {
                       ui.label("Método:"); ui.text_edit_singleline(&mut vb.bank_type); ui.end_row();
                       ui.label("Idioma:"); ui.text_edit_singleline(&mut vb.language); ui.end_row();
                       ui.label("Pasta:"); ui.text_edit_singleline(&mut vb.folder_name); ui.end_row();
                       ui.label("Link:"); ui.text_edit_singleline(&mut vb.download_link); ui.end_row();
                       ui.label("Imagem:"); ui.text_edit_singleline(&mut vb.image_path); ui.end_row();
                   });
                });
            });

            ui.add_space(30.0);
            if ui.button(egui::RichText::new("Remover Voicebank").color(egui::Color32::from_rgb(255, 80, 80))).clicked() {
                should_remove = true;
            }
        }

        if go_back {
            self.view_mode = ViewMode::Gallery;
        }

        if should_remove {
            self.voicebanks.remove(index);
            self.view_mode = ViewMode::Gallery;
        }
    }
}

fn ctx_if_hovered_pointer_cursor(ui: &mut egui::Ui) {
    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
}
