use egui::{Vec2, Widget};
use rand::prelude::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    
    #[serde(skip)]
    display: DisplayWidget
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,

            display: DisplayWidget::new(20, 64, 32),

        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::left("left test panel").show(ctx, |ui| {
            ui.add(egui::widgets::Label::new("this is a left sidepanel"));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("This is going to be a chip8-emulator");


            ui.add(&mut self.display);
            println!("{}", self.display.pixel_size);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
                
            });
        });
    }
}

struct DisplayWidget{
    pixel_size: usize,
    pix_x: usize,
    pix_y: usize,
    
    screen: Vec<bool>,
}

impl DisplayWidget {
    fn new(pixel_size: usize, pix_x: usize, pix_y: usize) -> Self{
        let mut screen: Vec<bool> = Vec::with_capacity(pix_x*pix_y);
        for _ in 0..(pix_x*pix_y){
            screen.push(rand::random());
        }
        DisplayWidget{pixel_size, pix_x, pix_y, screen}
    }

    fn fill_random(&mut self){
        self.screen.clear();
        for _ in 0..(self.pix_x*self.pix_y){
            self.screen.push(rand::random());
        }
    }
}

impl Widget for &mut DisplayWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.input(|i| if i.key_pressed(egui::Key::PlusEquals){
            self.pixel_size += 1;
        } else if i.key_pressed(egui::Key::Minus) {
            if self.pixel_size > 1{
                self.pixel_size -= 1;
            }
        } else if i.key_pressed(egui::Key::N){
            self.fill_random();
        });
        
        // allocate
        let (rect, response) = ui.allocate_exact_size(self.pixel_size as f32 *  (Vec2 { x: self.pix_x as f32, y: self.pix_y as f32}), egui::Sense{click: false, drag: false, focusable: false});
        
        // react to input
        

        // paint
        for i in 0..self.pix_x{
            for j in 0..self.pix_y{
                let mut color = egui::Color32::GREEN;
                if self.screen[i+j*self.pix_x]{
                    color = egui::Color32::BLACK;
                }

                
                let x_off = (i * self.pixel_size) as f32;
                let y_off = (j * self.pixel_size) as f32;
                // = min
                let mut upper_left = rect.min;
                upper_left.x += x_off;
                upper_left.y += y_off;
                // = max
                let mut lower_right = upper_left.clone();
                lower_right.x += self.pixel_size as f32;
                lower_right.y += self.pixel_size as f32;

                let pix_rec = egui::Rect::from_min_max(upper_left, lower_right);

                ui.painter().rect(pix_rec, egui::Rounding::ZERO, color, egui::Stroke::NONE);
            }
        }
        
        return response;
    }


    
}