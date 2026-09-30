use ratatui::{DefaultTerminal, Frame, layout::Rect, text, widgets::{Block, Paragraph, Wrap}};
use std::time::Duration;

fn main() {
    ratatui::run(app);
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {

    loop {
        terminal.draw(|frame| render(frame))?;

        if crossterm::event::poll(Duration::from_millis(80))?
            && crossterm::event::read()?.is_key_press()
        {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let text = "";

    let text_apps = Paragraph::new(text).block(Block::bordered()
        .title("app info"))
        .wrap(Wrap { trim: true });
    let text_find = Paragraph::new(" > \n app 1\n app 2\n app 3\n app 4\n app 5\n app 6\n app 7\n app 8\n app 9\n app 10\n app 11\n app 12\n app 13").block(Block::bordered()
        .title("find"))
        .wrap(Wrap { trim: true });
    let text_log = Paragraph::new("").block(Block::bordered()
        .title("log"))
        .wrap(Wrap { trim: true });

    frame.render_widget(text_apps, Rect::new(2, 0, 250, 20));
    frame.render_widget(text_find, Rect::new(2, 20, 80, 45));
    frame.render_widget(text_log, Rect::new(86, 20, 100, 45));
}