mod get_apps;
mod input_handler;
mod render_widgets;
mod get_apps_visual;
mod get_info;

use std::time::Duration;
use std::collections::HashMap;

use ratatui::{DefaultTerminal, widgets::ListState};

fn main() {
    let _ = ratatui::run(app);
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut log = String::new();
    let mut apps_visual = Vec::new();
    let (packages_tx, packages_rx) = std::sync::mpsc::channel();
    let (info_request_tx, info_request_rx) = std::sync::mpsc::channel::<String>();
    let (info_result_tx, info_result_rx) =
        std::sync::mpsc::channel::<(String, Result<Vec<String>, std::io::Error>)>();
    let info_delay = std::env::var("DNFINDER_INFO_DELAY_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(Duration::from_millis)
        .unwrap_or(Duration::from_millis(0));

    let mut current_filtered_app = String::new();
    let mut info_for_app = String::new();
    let mut app_info = Vec::new();

    std::thread::spawn(move || {
        let packages_visual = get_apps::get_packages();
        let _ = packages_tx.send(packages_visual);
    });

    std::thread::spawn(move || {
        let mut info_cache: HashMap<String, Vec<String>> = HashMap::new();

        while let Ok(mut package) = info_request_rx.recv() {
            loop {
                match info_request_rx.recv_timeout(info_delay) {
                    Ok(next_package) => package = next_package,
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => break,
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => return,
                }
            }

            let result = if let Some(info) = info_cache.get(&package) {
                Ok(info.clone())
            } else {
                let result = get_info::get_info(&package);
                if let Ok(info) = &result {
                    info_cache.insert(package.clone(), info.clone());
                }
                result
            };
            if info_result_tx.send((package, result)).is_err() {
                break;
            }
        }
    });

    let mut query = String::new();

    let mut list_state = ListState::default();

    loop {
        if let Ok(packages_visual) = packages_rx.try_recv() {
            if let Ok(packages_visual) = packages_visual {
                apps_visual = packages_visual;
            }
        }

        for (package, result) in info_result_rx.try_iter() {
            if package == current_filtered_app {
                app_info = result?;
            }
        }

        if current_filtered_app != info_for_app {
            app_info.clear();
            if !current_filtered_app.is_empty() {
                let _ = info_request_tx.send(current_filtered_app.clone());
            }
            info_for_app.clone_from(&current_filtered_app);
        }

        terminal.draw(|frame| {
            render_widgets::render(
                frame,
                &log,
                &apps_visual,
                &app_info,
                &query,
                &mut list_state,
                &mut current_filtered_app,
            );
        })?;

        match input_handler::inputs(&mut query, &mut list_state, &apps_visual, &mut current_filtered_app)? {
            Some(message) if message == "quit" => return Ok(()),
            Some(message) if message == "download" => {
                input_handler::handle_download(&current_filtered_app)?;
                return Ok(());
            }
            Some(message) => log = message,
            None => {}
        }
    }
}