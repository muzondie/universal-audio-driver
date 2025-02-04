use egui::{CentralPanel, ProgressBar, Button, Ui};
use crate::AppState;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct UiState {
    pub progress: f32,
    pub status: String,
    pub devices: Vec<crate::device_detection::AudioDevice>,
}

pub async fn run_ui(state: Arc<Mutex<AppState>>) -> Result<(), crate::error::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Universal Audio Driver",
        options,
        Box::new(|_| Box::new(MainWindow::new(state))),
    )
    .map_err(|e| crate::error::Error::GuiError(e.to_string()))
}

struct MainWindow {
    state: Arc<Mutex<AppState>>,
}

impl MainWindow {
    fn new(state: Arc<Mutex<AppState>>) -> Self {
        Self { state }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let state = self.state.lock();
            ui.heading("Audio Device Manager");
            
            if !state.devices.is_empty() {
                for device in &state.devices {
                    ui.label(format!("{} - {}", device.name, device.manufacturer));
                }
                
                if ui.add(Button::new("Install Drivers")).clicked() {
                    let state = self.state.clone();
                    tokio::spawn(async move {
                        let mut state = state.lock();
                        state.progress = 0.0;
                        state.status = "Installing...".into();
                        
                        for device in &mut state.devices {
                            state.progress += 0.1;
                            device.driver_status = crate::device_detection::DriverStatus::Installed;
                        }
                        
                        state.status = "Complete".into();
                    });
                }
            } else {
                ui.label("No audio devices detected");
            }
            
            ui.add(ProgressBar::new(state.progress).text(&state.status));
        });
    }
}