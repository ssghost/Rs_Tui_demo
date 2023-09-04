use std::{
    io,
    sync::{Arc, Mutex},
};
use tui::*;

pub mod tui;

pub struct AppState{
    pub decibels: Vec<f32>,
}

pub fn run_tui(
    state: Arc<Mutex<AppState>>,
) -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal, state)?;
    restore_terminal(&mut terminal)?;
    Ok(())
} 