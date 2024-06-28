//बिजी७७<bandesh@gmail.com>
use eframe::{egui};
use egui::{Ui, Widget, WidgetText, Layout, Button, TextEdit, Align, FontId, TextStyle, TextFormat, Color32,FontFamily, Response, Event, Key, vec2};
use egui::text::{LayoutSection, LayoutJob};
use std::sync::Arc;
use NepaliTransliterate::NepaliTransliterator;




struct MyApp  {
    input_text: String,
    output_text: String,
}

impl MyApp  {
    fn new() -> Self {
        Self {
            input_text: "".to_string(),
            output_text: "".to_string(),
        }
    }
}




fn main() {
    let app = MyApp::new();
      let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Nepali Transliterator",
        options,
        Box::new(|cc| Box::new(app)),
    );
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Input:");
                    let edit = UnicodeTextEdit::new(self.input_text.clone(), 200.0, 100.0);
                    let response = ui.add(edit);
                    if response.changed() {
                        self.input_text = response.ctx.input(|i| i.raw.events.iter().filter_map(|event| {
                            if let egui::Event::Text(text) = event {
                                Some(text.clone())
                            } else {
                                None
                            }
                        }).collect::<String>());
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Output:");
                    let edit = UnicodeTextEdit::new(self.output_text.clone(), 200.0, 100.0);
                    let response = ui.add(edit);
                    if response.changed() {
                        self.output_text = response.ctx.input(|i| i.raw.events.iter().filter_map(|event| {
                            if let egui::Event::Text(text) = event {
                                Some(text.clone())
                            } else {
                                None
                            }
                        }).collect::<String>());
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("To Nepali").clicked() {
                        let transliterator = NepaliTransliterator::new();
                        self.output_text = transliterator.to_nepali(&self.input_text);
                    }

                    if ui.button("To Roman").clicked() {
                        let transliterator = NepaliTransliterator::new();
                        self.output_text = transliterator.to_roman(&self.input_text);
                    }
                });
            });
        });
    }
}

struct UnicodeTextEdit {
    text: String,
    width: f32,
    height: f32,
}

 


impl UnicodeTextEdit {
    fn new(text: String, width: f32, height: f32) -> Self {
        Self { text, width, height }
    }

    fn text(&self) -> &str {
        &self.text
    }
}

impl Widget for UnicodeTextEdit {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let font_id = FontId::new(20.0, FontFamily::Proportional);
        let text_color = Color32::BLACK;
        let bg_color = Color32::WHITE;
        let border_color = Color32::GRAY;
        let text_format = egui::TextFormat::simple(font_id.clone(), text_color);

        let mut layout_job = LayoutJob::default();
        layout_job.append(&self.text, 0.0, text_format);
        layout_job.wrap.max_width = self.width;

        let galley = ui.fonts(|fonts| fonts.layout_job(layout_job));
        let desired_size = vec2(self.width, self.height);

        let (mut response, painter) = ui.allocate_painter(desired_size, egui::Sense::click_and_drag());
        let rect = response.rect;

        painter.rect_filled(rect, 0.0, bg_color);
        painter.rect_stroke(rect, 0.0, (1.0, border_color));
        painter.galley(rect.min + vec2(5.0, 5.0), Arc::clone(&galley), text_color);

        if response.clicked() {
            ui.memory_mut(|mem| mem.request_focus(response.id));
        }

        let mut changed = false;
        let has_focus = ui.memory(|mem| mem.has_focus(response.id));

        if has_focus {
            let cursor_color = Color32::BLACK;

            ui.input_mut(|input| {
                for event in input.events.iter() {
                    match event {
                        egui::Event::Text(text) => {
                            self.text.push_str(text);
                            changed = true;
                        }
                        egui::Event::Key { key, pressed: true, .. } => {
                            match key {
                                egui::Key::Enter => {
                                    self.text.push('\n');
                                    changed = true;
                                }
                                egui::Key::Backspace => {
                                    self.text.pop();
                                    changed = true;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            });

            // Draw the cursor
            let cursor_pos = rect.min + vec2(5.0, 5.0 + galley.size().y);
            painter.line_segment([cursor_pos, cursor_pos + vec2(0.0, -20.0)], (1.0, cursor_color));
        }

        if changed {
            response.mark_changed();
        }

        response
    }
}