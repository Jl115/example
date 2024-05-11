// Configure the build to run on Windows with the `windows_subsystem` feature
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui; // Import egui for GUI-related functionality
use egui::Color32; // Import Color32 struct from egui
use rand::Rng; // Import random number generator

use std::{
    sync::{Arc, Mutex}, // Import Arc and Mutex structs from the standard library's `sync` module
    thread,             // Import the thread module for working with threads
    time::Duration,     // Import Duration type for timing-related functionality
};

const WIDTH: usize = 60; // Define constant for the width of the display
const HEIGHT: usize = 60; // Define constant for the height of the display

// Define a struct `App` that holds an arc-mutex pair containing a vector of colors
struct App {
    colors: Arc<Mutex<Vec<Color32>>>,
}

impl App {
    // Implement a method to create a new instance of `App`
    fn new() -> Self {
        Self {
            colors: Arc::new(Mutex::new(vec![Color32::WHITE; WIDTH * HEIGHT])),
        }
    }
}

// Implement the `eframe::App` trait for `App`
impl eframe::App for App {
    // Update the GUI when a frame is received
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Lock and retrieve the colors vector from `self.colors`
            let mut colors = self.colors.lock().unwrap();
            for y in 0..HEIGHT {
                ui.horizontal(|ui| {
                    // Iterate over each row and column
                    for x in 0..WIDTH {
                        let idx = x + y * WIDTH;
                        // Get the current color from the vector at the specified index
                        let color = colors[idx];
                        // Draw a colored label with the symbol "■"
                        ui.colored_label(color, "■");
                    }
                });
            }
        });
    }
}

// Define a function to start a new thread that updates the color of a specific index in the `colors` vector
fn start_color_update_thread(colors: Arc<Mutex<Vec<Color32>>>, index: usize) {
    // Spawn a new thread and move the closure into it
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            // Sleep for a random duration between 500-1500 milliseconds
            thread::sleep(Duration::from_millis(rng.gen_range(500..1500)));
            // Generate a new random color and update the vector at the specified index
            let color = Color32::from_rgb(rng.gen(), rng.gen(), rng.gen());
            let mut colors_lock = colors.lock().unwrap();
            colors_lock[index] = color;
        }
    });
}

// Define the `main` function that sets up the GUI application
fn main() {
    // Create a new instance of `NativeOptions`
    let options = eframe::NativeOptions::default();
    // Create a new instance of `App`
    let app = App::new();
    // Clone the colors vector and pass it to the thread spawning function
    let colors_clone = Arc::clone(&app.colors);
    for i in 0..WIDTH * HEIGHT {
        start_color_update_thread(Arc::clone(&colors_clone), i); // Pass the cloned colors vector to each thread
    }
    // Run the GUI application with the specified title and options
    eframe::run_native("Threads Blocks", options, Box::new(|_cc| Box::new(app)));
}
