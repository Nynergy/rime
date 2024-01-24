use std::{
    cmp,
    ffi::OsStr,
};
use tui::{
    backend::Backend,
    buffer::Buffer,
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
        Margin,
        Rect,
    },
    style::{Color, Modifier, Style},
    symbols::line,
    text::{Span, Spans},
    widgets::{
        Block,
        Clear,
        List,
        ListItem,
        ListState,
        Paragraph,
        Widget,
    },
    Frame,
};

use crate::app::*;

macro_rules! raw_para {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_para = Vec::new();
            $(
                temp_para.push(
                    Spans::from(
                        Span::raw($x)
                    )
                );
            )*
            temp_para
        }
    };
}

struct CustomBorder {
    title: String,
    title_style: Style,
    border_style: Style,
}

impl CustomBorder {
    fn new() -> Self {
        Self {
            title: "".to_string(),
            title_style: Style::default(),
            border_style: Style::default(),
        }
    }

    fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
}

impl Widget for CustomBorder {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Border Lines
        let mut line = String::new();
        line.push_str(line::VERTICAL_RIGHT);
        for _ in 0..area.width - 2 {
            line.push_str(line::HORIZONTAL);
        }
        line.push_str(line::VERTICAL_LEFT);
        buf.set_string(area.left(), area.top(), line.clone(), self.border_style);
        buf.set_string(area.left(), area.bottom() - 1, line, self.border_style);

        // Title
        let offset = area.width / 2 - self.title.len() as u16 / 2;
        let title_x = area.left() + offset;
        let title_y = area.y;
        buf.set_string(title_x, title_y, self.title.clone(), self.title_style);

        // Title Tee's
        buf.set_string(
            title_x - 1,
            area.top(),
            line::VERTICAL_LEFT,
            self.border_style
        );
        buf.set_string(
            title_x + self.title.len() as u16,
            area.top(),
            line::VERTICAL_RIGHT,
            self.border_style
        );
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    if (f.size().height < 9) || (f.size().width < 20) {
        f.render_widget(Clear, f.size());
        return;
    }

    match app.state {
        AppState::FileNavigation => render_main_interface(f, app),
    }
}

fn render_main_interface<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    if f.size().height > 50 {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                Constraint::Length(10),
                Constraint::Min(3),
                Constraint::Length(3),
                ]
                .as_ref()
            )
            .split(f.size());

        render_banner(f, chunks[0]);

        if f.size().width > 120 {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Min(10),
                        Constraint::Percentage(75),
                    ]
                    .as_ref()
                )
                .split(chunks[1]);

            render_file_navigator(f, chunks[0], app);
            render_tag_columns(f, chunks[1], app);
        } else {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Min(5),
                        Constraint::Percentage(65),
                    ]
                    .as_ref()
                )
                .split(chunks[1]);

            render_file_navigator(f, chunks[0], app);
            render_tag_columns(f, chunks[1], app);
        }

        render_footer_info(f, chunks[2]);
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                Constraint::Length(5),
                Constraint::Min(3),
                Constraint::Length(1),
                ]
                .as_ref()
            )
            .split(f.size());

        render_tiny_banner(f, chunks[0]);

        if f.size().width > 120 {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Min(10),
                        Constraint::Percentage(75),
                    ]
                    .as_ref()
                )
                .split(chunks[1]);

            render_file_navigator(f, chunks[0], app);
            render_tag_columns(f, chunks[1], app);
        } else {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Min(5),
                        Constraint::Percentage(65),
                    ]
                    .as_ref()
                )
                .split(chunks[1]);

            render_file_navigator(f, chunks[0], app);
            render_tag_columns(f, chunks[1], app);
        }

        render_empty_line(f, chunks[2]);
    }
}

fn render_banner<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
) {
    let banner = raw_para!(
        "",
        "         _              ",
        "   _____(_)___ ___  ___ ",
        "  / ___/ / __ `__ \\/ _ \\",
        " / /  / / / / / / /  __/",
        "/_/  /_/_/ /_/ /_/\\___/ ",
        "",
        "An id3 tag editor for the Terminal",
        ""
    );

    let banner = Paragraph::new(banner)
        .block(Block::default())
        .style(
            Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
        )
        .alignment(Alignment::Center);

    f.render_widget(banner, chunk);
}

fn render_tiny_banner<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
) {
    let banner = raw_para!(
        "",
        "rime",
        "",
        "An id3 tag editor for the Terminal",
        ""
    );

    let banner = Paragraph::new(banner)
        .block(Block::default())
        .style(
            Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
        )
        .alignment(Alignment::Center);

    f.render_widget(banner, chunk);
}

fn render_footer_info<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let info = raw_para!(
        "",
        "rime v1.0.0 by Ben Buchanan (https://github.com/Nynergy)"
    );

    let info = Paragraph::new(info)
        .block(Block::default())
        .alignment(Alignment::Center);

    f.render_widget(info, chunk);
}

fn render_empty_line<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let line = raw_para!("");
    let line = Paragraph::new(line)
        .block(Block::default());

    f.render_widget(line, chunk);
}

fn render_file_navigator<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    app: &mut App
) {
    render_column_block(f, chunk, "File Navigator".to_string());

    let mut item_style = Style::default();

    let items: Vec<ListItem> = app
        .pwd
        .items
        .iter()
        .map(|i| {
            item_style = Style::reset();

            if i.is_dir() {
                item_style = item_style.fg(Color::Cyan);
            } else {
                item_style = item_style.fg(Color::Magenta);
            }

            if let Some(_) = app.selected_files.get(i) {
                item_style = item_style
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD);
            }

            let raw_file_name = i.file_name()
                  // Handle parent directory
                  .unwrap_or(OsStr::new(".."))
                  .to_str()
                  .unwrap();

            let chopped_file_name = truncate_text(raw_file_name.to_string(),
                                                  (chunk.width - 2) as usize);

            ListItem::new(chopped_file_name)
                .style(item_style)
        })
        .collect();

    let highlight = Style::default()
        .add_modifier(Modifier::REVERSED);

    let list = List::new(items)
        .block(Block::default())
        .highlight_style(highlight);

    let inner_area = shrink_rect(chunk, 1);

    f.render_stateful_widget(
        list,
        inner_area,
        &mut app.pwd.state
    );
}

fn render_tag_columns<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    app: &App
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref()
        )
        .split(chunk);

    render_column_block(
        f, chunks[0],
        format!("Current Tags ({} Files Selected)", app.num_selected_files())
    );
    render_tag_list(f, chunks[0], app);

    render_column_block(f, chunks[1], "New Tags".to_string());
}

fn render_column_block<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    title: String
) {
    let container = CustomBorder::new()
        .title(title.to_string());

    f.render_widget(container, chunk);
}

fn render_tag_list<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    app: &App
) {
    let inner_area = shrink_rect(chunk, 1);

    // TODO: Sort tag frames before rendering them
    let items: Vec<ListItem> = app.tag_sum
        .iter()
        .map(|(k, v)| {
            ListItem::new(
                tag_span(
                    translate_tag_id(k),
                    v.to_string(),
                    inner_area.width
                )
            )
        })
        .collect();

    let list = List::new(items)
        .block(Block::default());

    f.render_stateful_widget(
        list,
        inner_area,
        &mut ListState::default()
    );
}

fn tag_span<'a>(
    name: String,
    value: String,
    width: u16
) -> Vec<Spans<'a>> {
    let mut lines = Vec::new();

    create_top_line(&mut lines, width);
    create_middle_line(&mut lines, width, name, value);
    create_bottom_line(&mut lines, width);

    lines
}

fn create_top_line(lines: &mut Vec<Spans>, width: u16) {
    let mut line = String::from(line::TOP_LEFT);
    for _ in 0..width - 2 {
        line.push_str(line::HORIZONTAL);
    }
    line.push_str(line::TOP_RIGHT);
    lines.push(Spans::from(line));
}

fn create_middle_line(
    lines: &mut Vec<Spans>,
    width: u16,
    name: String,
    value: String
) {
    // Left Side
    let line = String::from(
        format!("{} ", line::VERTICAL)
    );
    let mut spans = vec![Span::raw(line)];

    // Name
    spans.push(Span::styled(
            format!("{}: ", name),
            Style::default()
            .add_modifier(Modifier::BOLD)
    ));

    // Value
    spans.push(Span::raw(value));

    // Padding Spaces
    let current_width = spans
        .iter()
        .map(|span| span.width())
        .sum::<usize>();
    let remaining_width = cmp::max(
        ((width - 2) as usize).checked_sub(current_width)
            .unwrap_or(1),
        1
    );

    let mut line = String::new();
    for _ in 0..remaining_width {
        line.push_str(" ");
    }
    spans.push(Span::raw(line));

    // Right Side
    let line = String::from(
        format!(" {}", line::VERTICAL)
    );
    spans.push(Span::raw(line));
    lines.push(Spans::from(spans));
}

fn create_bottom_line(lines: &mut Vec<Spans>, width: u16) {
    let mut line = String::from(line::BOTTOM_LEFT);
    for _ in 0..width - 2 {
        line.push_str(line::HORIZONTAL);
    }
    line.push_str(line::BOTTOM_RIGHT);
    lines.push(Spans::from(line));
}

fn translate_tag_id(id: &str) -> String {
    match id {
        "APIC" => String::from("Image       "),
        "COMM" => String::from("Comment     "),
        "TALB" => String::from("Album       "),
        "TCON" => String::from("Genre       "),
        "TIT2" => String::from("Title       "),
        "TPE1" => String::from("Artist      "),
        "TPE2" => String::from("Album Artist"),
        "TPOS" => String::from("Disc        "),
        "TRCK" => String::from("Track       "),
        "TSRC" => String::from("ISRC        "),
        "TSSE" => String::from("Encoding    "),
        "TYER" => String::from("Date        "),
        // TODO: We may not want to handle custom frame data
        "TXXX" => String::from("Custom Frame"),
        "USLT" => String::from("Lyrics      "),
        _ => panic!("Could not translate id3 frame '{}'", id)
    }
}

fn shrink_rect(rect: Rect, amount: u16) -> Rect {
    let margin = Margin { vertical: amount, horizontal: amount };
    rect.inner(&margin)
}

fn truncate_text(mut text: String, max_length: usize) -> String {
    if text.len() > max_length {
        text.truncate(max_length - 3);
        return format!("{}...", text);
    }

    text
}
