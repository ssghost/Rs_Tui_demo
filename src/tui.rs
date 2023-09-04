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
    io::{self, Stdut},
    sync::{Arc, Mutex},
    time::Duration,
};
use crate::AppState;


