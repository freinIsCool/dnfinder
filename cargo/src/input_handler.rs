use std::{io, process::Command};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn inputs() -> io::Result<Option<String>> {
    if !event::poll(Duration::from_millis(0))? {
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
                return Ok(Some(String::from_utf8_lossy(
                    &Command::new("sudo").args(&["dnf", "install", "firefox"]).output().unwrap().stdout,
                )
                .to_string()));
            }
            return Ok(Some(format!("key {}, pressed!", key.code.to_string())));
        }
    }

    Ok(None)
}