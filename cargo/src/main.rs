mod get_apps;
mod input_handler;
mod render_widgets;
mod get_apps_visual;

use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout, Rect}, widgets::{Block, Paragraph, Wrap}};

fn main() {
    let _ = ratatui::run(app);
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut log = String::new();
    let mut apps_visual = Vec::new();
    let query = String::new();
    let (packages_tx, packages_rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let packages_visual = get_apps::get_packages();
        let _ = packages_tx.send(packages_visual);
    });

    let mut query = String::new();

    loop {
        terminal.draw(|frame| {
            render_widgets::render(frame, &log, &apps_visual, &query);
        })?;

        input_handler::inputs(&mut query)?;

        if let Ok(packages_visual) = packages_rx.try_recv() {
            if let Ok(packages_visual) = packages_visual {
                apps_visual = packages_visual;
            }
        }

        terminal.draw(|frame| {
            render_widgets::render(frame, &log, &apps_visual, &query);
        })?;

        match input_handler::inputs(&mut query)? {
            Some(message) if message == "quit" => return Ok(()),
            Some(message) => log = message,
            None => {}
        }
    }
}