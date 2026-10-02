use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn render(
        frame: &mut Frame,
        log: &str,
        apps_visual: &[String],
        info: &[String],
        query: &str,
        list_state: &mut ListState,
        current_filtered_app: &mut String,
    ) {

    let area = frame.area();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(20), // app info
            Constraint::Min(1),     // find
        ])
        .split(frame.area());

    let bottom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(100),
            Constraint::Min(1),
        ])
        .split(main_layout[1]);

        let find_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // search box
                Constraint::Min(1),    // app list
            ])
            .split(main_layout[1]);

    let search = Paragraph::new(format!("> {}", query))
        .block(Block::bordered().title("find"));

    frame.render_widget(search, find_layout[0]);

    frame.set_cursor_position((
        find_layout[0].x + 3 + query.len() as u16,
        find_layout[0].y + 1,
    ));

    let app_info = Paragraph::new(info.join("\n"))
        .block(
            Block::bordered()
                .title("app info")
        )
        .wrap(Wrap { trim: true });

    let search = Paragraph::new(format!("> {}", query))
        .block(Block::bordered().title("find"));

    let filtered_apps: Vec<&String> = if query.is_empty() {
        Vec::new()
    } else {
        apps_visual.iter()
            .filter(|app| app.contains(query))
            .collect()
    };

    if filtered_apps.is_empty() {
        list_state.select(None);
    } else if list_state.selected().is_none_or(|index| index >= filtered_apps.len()) {
        list_state.select(Some(0));
    }

    if let Some(selected) = list_state.selected() {
        current_filtered_app.clone_from(filtered_apps[selected]);
    } else {
        current_filtered_app.clear();
    }

    let items: Vec<ListItem> = filtered_apps
        .iter()
        .map(|app| ListItem::new(app.as_str()))
        .collect();

    let app_list = List::new(items)
        .block(Block::bordered())
        .highlight_symbol("> ");

    frame.render_widget(app_info, main_layout[0]);
    frame.render_widget(search, find_layout[0]);
    frame.render_stateful_widget(app_list, find_layout[1], list_state);
}