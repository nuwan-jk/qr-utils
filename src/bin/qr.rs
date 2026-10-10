#![windows_subsystem = "windows"]

use eframe::egui;
use qrcode::QrCode;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 60.0])
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    eframe::run_native(
        "RQ",
        options,
        Box::new(|_cc| Box::new(RqApp::default())),
    )
}

struct RqApp {
    input_text: String,
    texture: Option<egui::TextureHandle>,
}

impl Default for RqApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            texture: None,
        }
    }
}

impl eframe::App for RqApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            if self.texture.is_none() {
                if !self.input_text.is_empty() {
                    if let Ok(code) = QrCode::new(self.input_text.as_bytes()) {
                        let bit_array = code.to_colors();
                        let width = code.width();
                        let scale = 8;
                        let margin = 4;
                        let img_width = (width + margin * 2) * scale;
                        
                        let mut pixels = Vec::with_capacity(img_width * img_width);
                        for y in 0..img_width {
                            for x in 0..img_width {
                                let block_x = x / scale;
                                let block_y = y / scale;
                                if block_x < margin || block_y < margin || block_x >= width + margin || block_y >= width + margin {
                                    pixels.push(egui::Color32::WHITE);
                                } else {
                                    let color = bit_array[(block_y - margin) * width + (block_x - margin)];
                                    if color == qrcode::Color::Dark {
                                        pixels.push(egui::Color32::BLACK);
                                    } else {
                                        pixels.push(egui::Color32::WHITE);
                                    }
                                }
                            }
                        }
                        let color_image = egui::ColorImage {
                            size: [img_width, img_width],
                            pixels,
                        };
                        self.texture = Some(ctx.load_texture("qr", color_image, egui::TextureOptions::NEAREST));
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(img_width as f32, img_width as f32)));
                    }
                }
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }

        if self.texture.is_none() {
            egui::CentralPanel::default().frame(egui::Frame::none().inner_margin(10.0)).show(ctx, |ui| {
                let response = ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::singleline(&mut self.input_text)
                        .hint_text("Type and press Enter...")
                );
                response.request_focus();
            });
        } else {
            egui::CentralPanel::default().frame(egui::Frame::none().fill(egui::Color32::WHITE)).show(ctx, |ui| {
                if let Some(texture) = &self.texture {
                    ui.centered_and_justified(|ui| {
                        ui.image((texture.id(), texture.size_vec2()));
                    });
                }
            });
        }
    }
}
