use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Device detection failed")]
    DetectionFailure,
    
    #[error("Driver download failed")]
    DownloadFailed,
    
    #[error("Installation failed")]
    InstallationFailed,
    
    #[error("Windows API error")]
    WindowsError(#[from] windows::core::Error),
    
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    
    #[error("GUI error: {0}")]
    GuiError(String),
}