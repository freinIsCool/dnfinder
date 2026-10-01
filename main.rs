mod get_apps_visual;
mod get_apps;
mod input_handler;

use ratatui::{DefaultTerminal, Frame, layout::Rect, widgets::{Block, Paragraph, Wrap}};

fn main() {
    let _ = ratatui::run(app);
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut log = String::new();

    loop {
        terminal.draw(|frame| render(frame, &log))?;

        match input_handler::inputs()? {
            Some(message) if message == "quit" => return Ok(()),
            Some(message) => log = message,
            None => {}
        }
    }
}

fn render(frame: &mut Frame, log: &str) {
    let apps: String = get_apps_visual::get_packages_string().unwrap_or_default();
    let apps_format = format!(">\n {}", apps);

    let text_apps = Paragraph::new("").block(Block::bordered()
        .title("app info"))
        .wrap(Wrap { trim: true });
    let text_find = Paragraph::new(apps_format).block(Block::bordered()
        .title("find"))
        .wrap(Wrap { trim: true });
    let text_log = Paragraph::new(log).block(Block::bordered()
        .title("log"))
        .wrap(Wrap { trim: true });

    frame.render_widget(text_apps, Rect::new(2, 0, 250, 20));
    frame.render_widget(text_find, Rect::new(2, 20, 80, 45));
    frame.render_widget(text_log, Rect::new(86, 20, 100, 45));
}