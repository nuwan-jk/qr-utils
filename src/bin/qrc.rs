#![windows_subsystem = "windows"]

use arboard::Clipboard;
use eframe::egui;
use qrcode::QrCode;

fn main() -> Result<(), eframe::Error> {
    let text = match Clipboard::new() {
        Ok(mut cb) => cb.get_text().unwrap_or_else(|_| String::new()),
        Err(_) => String::new(),
    };

    let (image, win_size, has_text) = generate_qr_image(&text);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(win_size)
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    eframe::run_native(
        "QRC",
        options,
        Box::new(move |cc| {
            let texture = if let Some(img) = image {
                Some(cc.egui_ctx.load_texture("qr", img, egui::TextureOptions::NEAREST))
            } else {
                None
            };
            Box::new(QrApp { texture, has_text })
        }),
    )
}

fn generate_qr_image(text: &str) -> (Option<egui::ColorImage>, egui::Vec2, bool) {
    if text.is_empty() {
        return (None, egui::vec2(300.0, 100.0), false);
    }
    if let Ok(code) = QrCode::new(text.as_bytes()) {
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
        let image = egui::ColorImage {
            size: [img_width, img_width],
            pixels,
        };
        (Some(image), egui::vec2(img_width as f32, img_width as f32), true)
    } else {
        (None, egui::vec2(300.0, 100.0), true)
    }
}

struct QrApp {
    texture: Option<egui::TextureHandle>,
    has_text: bool,
}

impl eframe::App for QrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Enter)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::CentralPanel::default().frame(egui::Frame::none().fill(egui::Color32::WHITE)).show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                ui.centered_and_justified(|ui| {
                    ui.image((texture.id(), texture.size_vec2()));
                });
            } else {
                ui.centered_and_justified(|ui| {
                    if !self.has_text {
                        ui.heading("Clipboard is empty!");
                    } else {
                        ui.heading("Failed to generate QR!");
                    }
                });
            }
        });
    }
}
