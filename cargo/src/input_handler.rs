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
    ratatui::restore();
    ratatui::init();

    Ok(Some("Download finished!\n\npress q or esc to quit".to_string()))
}

pub fn inputs() -> io::Result<Option<String>> {
    if !event::poll(Duration::from_millis(50))? {
        return Ok(None);
    }

    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                return Ok(Some("quit".to_string()));
            }
            if key.code == KeyCode::Left {
                return Ok(Some("key left pressed".to_string()));
            }
            if key.code == KeyCode::Enter {
                return handle_download();
            }
            return Ok(Some(format!("key {}, pressed!", key.code.to_string())));
        }
    }

    Ok(None)
}
