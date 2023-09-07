use std::{
    io,
    sync::{Arc, Mutex},
};
use tui::*;
use std::path::Path;
use wavers::{read, SampleType};

pub mod tui;

pub struct AppState{
    pub decibels: Vec<f32>,
}

pub fn read_wave(input: Path) -> Arc<Mutex<AppState>> {
    let mut state = AppState::new(read(Path::new("*.wav"), SampleType::F32(0.0)).unwrap());
    Ok(state)
} 

pub fn run_tui(
    state: Arc<Mutex<AppState>>,
) -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal, state)?;
    restore_terminal(&mut terminal)?;
    Ok(())
} 