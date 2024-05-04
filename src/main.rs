#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::Color32;
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const WIDTH: usize = 60;
const HEIGHT: usize = 60;

struct App {
    colors: Arc<Mutex<Vec<Color32>>>,
}

impl App {
    fn new() -> Self {
        Self {
            colors: Arc::new(Mutex::new(vec![Color32::WHITE; WIDTH * HEIGHT])),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let colors = self.colors.lock().unwrap();
            for y in 0..HEIGHT {
                ui.horizontal(|ui| {
                    for x in 0..WIDTH {
                        let idx = x + y * WIDTH;
                        let color = colors[idx];
                        ui.colored_label(color, "■");
                    }
                });
            }
        });
    }
}

fn start_color_update_thread(colors: Arc<Mutex<Vec<Color32>>>, index: usize) {
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            thread::sleep(Duration::from_millis(rng.gen_range(500..1500)));
            let color = Color32::from_rgb(rng.gen(), rng.gen(), rng.gen());
            let mut colors_lock = colors.lock().unwrap();
            colors_lock[index] = color;
        }
    });
}

fn main() {
    let options = eframe::NativeOptions::default();
    let app = App::new();
    let colors_clone = Arc::clone(&app.colors);

    for i in 0..WIDTH * HEIGHT {
        start_color_update_thread(Arc::clone(&colors_clone), i);
    }

    eframe::run_native("Threads Blocks", options, Box::new(|_cc| Box::new(app)));
}
