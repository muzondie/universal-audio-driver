use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger, TermLogger, TerminalMode};
use std::fs::File;

pub fn init() -> Result<(), crate::error::Error> {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("audio-driver.log")?
        ),
    ])?;
    Ok(())
}