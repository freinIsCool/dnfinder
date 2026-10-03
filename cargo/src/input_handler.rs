use std::{io, process::Command, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::ListState;

pub fn handle_download(app_to_install: &str) -> io::Result<()> {
    ratatui::restore();

    if app_to_install.is_empty() {
        return Err(io::Error::other("No app selected for download"));
    }

    Command::new("sudo")
        .args(["dnf", "install", app_to_install])
        .status()?;

    Ok(())
}

pub const fn is_ascii_alphanumeric_and_dash(c: char) -> bool {
    matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z' | '-')
}

pub fn inputs(
    query: &mut String,
    list_state: &mut ListState,
    apps_visual: &[String],
    current_filtered_app: &mut String,
) -> io::Result<Option<String>> {
    if !event::poll(Duration::from_millis(50))? {
        return Ok(None);
    }

    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char(c) if is_ascii_alphanumeric_and_dash(c) => {
                    query.push(c);
                    list_state.select(Some(0));
                }

                KeyCode::Backspace => {
                    query.pop();
                    list_state.select(Some(0));
                }

                KeyCode::Esc => {
                    return Ok(Some("quit".to_string()));
                }

                KeyCode::Enter => {
                    if !current_filtered_app.is_empty() {
                        return Ok(Some("download".to_string()));
                    }
                }

                KeyCode::Up => {
                    select_matching_app(query, list_state, apps_visual, current_filtered_app, false);
                }

                KeyCode::Down => {
                    select_matching_app(query, list_state, apps_visual, current_filtered_app, true);
                }

                _ => {}
            }

            if matches!(key.code, KeyCode::Char(_) | KeyCode::Backspace) {
                sync_selected_app(query, list_state, apps_visual, current_filtered_app);
            }
        }
    }

    Ok(None)
}

fn select_matching_app(
    query: &str,
    list_state: &mut ListState,
    apps_visual: &[String],
    current_filtered_app: &mut String,
    move_down: bool,
) {
    let filtered_apps: Vec<&String> = apps_visual
        .iter()
        .filter(|app| app.contains(query))
        .collect();

    if filtered_apps.is_empty() {
        list_state.select(None);
        current_filtered_app.clear();
        return;
    }

    let selected = list_state
        .selected()
        .unwrap_or(0)
        .min(filtered_apps.len() - 1);
    let selected = if move_down {
        (selected + 1).min(filtered_apps.len() - 1)
    } else {
        selected.saturating_sub(1)
    };

    list_state.select(Some(selected));
    current_filtered_app.clone_from(filtered_apps[selected]);
}

fn sync_selected_app(
    query: &str,
    list_state: &mut ListState,
    apps_visual: &[String],
    current_filtered_app: &mut String,
) {
    let filtered_apps: Vec<&String> = apps_visual
        .iter()
        .filter(|app| app.contains(query))
        .collect();

    if filtered_apps.is_empty() {
        list_state.select(None);
        current_filtered_app.clear();
        return;
    }

    list_state.select(Some(0));
    current_filtered_app.clone_from(filtered_apps[0]);
}
