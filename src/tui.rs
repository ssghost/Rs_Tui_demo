use crossterm::{
    event::{self, Event, Keycode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use miette::Result;
use ratatui::{prelude::*, widgets::*};
use std::{
    io::{self, Stdout},
    sync::{Arc, Mutex},
    time::Duration,
};
use crate::AppState;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(
        Terminal::new(CrosstermBackend::new(stdout,))?
    )
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    Ok(terminal.show_cursor()?)
}

