mod device_detection;
mod driver_management;
mod gui;
mod logging;
mod error;

use crate::gui::AppState;
use error::Error;
use log::info;
use parking_lot::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    logging::init()?;
    info!("Starting Universal Audio Driver");

    let state = Arc::new(Mutex::new(AppState::default()));
    gui::run_ui(state).await
}