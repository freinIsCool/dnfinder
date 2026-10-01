use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame, log: &str, apps_visual: &[String], query: &str) {
    let area = frame.area();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(20),
            Constraint::Min(1),
        ])
        .split(area);

    let bottom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Percentage(55),
        ])
        .split(main_layout[1]);

    let find_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(bottom_layout[0]);

    let search = Paragraph::new(format!("> {}", query))
        .block(Block::bordered().title("find"));

    frame.render_widget(search, find_layout[0]);

    frame.set_cursor_position((
        find_layout[0].x + 3 + query.len() as u16,
        find_layout[0].y + 1,
    ));

    let app_info = Paragraph::new("")
        .block(
            Block::bordered()
                .title("app info")
        )
        .wrap(Wrap { trim: true });

    let search = Paragraph::new(format!("> {}", query))
        .block(Block::bordered().title("find"));

    let items: Vec<ListItem> = apps_visual
        .iter()
        .filter(|app| !app.trim().is_empty() && app.contains(query))
        .map(|app| ListItem::new(Line::from(app.as_str())))
        .collect();

    let app_list = List::new(items)
        .block(Block::bordered());

    let text_log = Paragraph::new(log)
        .block(Block::bordered().title("log"))
        .wrap(Wrap { trim: true });

    frame.render_widget(app_info, main_layout[0]);
    frame.render_widget(search, find_layout[0]);
    frame.render_widget(app_list, find_layout[1]);
    frame.render_widget(text_log, bottom_layout[1]);
}