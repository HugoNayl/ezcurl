use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::Stdout;

use crate::error::EzcurlError;

pub type AppTerminal = Terminal<CrosstermBackend<Stdout>>;

pub fn setup_terminal() -> Result<AppTerminal, EzcurlError> {
    // ratatui '*init' methods also setup panic hooks for error handling
    Ok(ratatui::try_init()?)
}

pub fn exit_terminal(terminal: &mut AppTerminal) -> Result<(), EzcurlError> {
    ratatui::try_restore()?;
    terminal.show_cursor()?;
    Ok(())
}
