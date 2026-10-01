use std::{io, process::Command};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::get_apps;

pub fn handle_download() -> io::Result<Option<String>> {
    ratatui::restore();
    ratatui::init();

    Command::new("sudo")
        .args(["dnf", "install", "firefox"])
        .status()?;

    return Ok(Some("quit".to_string()));
}

pub fn inputs(query: &mut String) -> io::Result<Option<String>> {
    if !event::poll(Duration::from_millis(50))? {
        return Ok(None);
    }

    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char(c) if c.is_ascii_alphabetic() => {
                    query.push(c);
                }

                KeyCode::Backspace => {
                    query.pop();
                }

                KeyCode::Esc | KeyCode::Esc => {
                    return Ok(Some("quit".to_string()));
                }

                KeyCode::Enter => {
                    return handle_download();
                }

                _ => {}
            }
        }
    }

    Ok(None)
}
