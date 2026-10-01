mod get_apps_visual;
mod get_apps;
mod input_handler;
mod render_widgets;

use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout, Rect}, widgets::{Block, Paragraph, Wrap}};

fn main() {
    let _ = ratatui::run(app);
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut log = String::new();
    let mut apps = String::new();
    let query = String::new();
    let (packages_tx, packages_rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = packages_tx.send(get_apps_visual::get_packages_string());
    });

    loop {
        if let Ok(Ok(packages)) = packages_rx.try_recv() {
            apps = packages;
        }

        terminal.draw(|frame| {
            render_widgets::render(frame, &log, &apps, &query);
        })?;

        match input_handler::inputs()? {
            Some(message) if message == "quit" => return Ok(()),
            Some(message) => log = message,
            None => {}
        }
    }
}