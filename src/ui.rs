use tui::{
    backend::Backend,
    buffer::Buffer,
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{Color, Modifier, Style},
    symbols::line,
    text::{Span, Spans},
    widgets::{
        Block,
        Clear,
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
        AppState::TagEditor => render_tag_editor(f, app),
    }
}

fn render_tag_editor<B: Backend>(f: &mut Frame<B>, _app: &mut App) {
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

            render_file_navigator(f, chunks[0]);
            render_tag_columns(f, chunks[1]);
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

            render_file_navigator(f, chunks[0]);
            render_tag_columns(f, chunks[1]);
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

            render_file_navigator(f, chunks[0]);
            render_tag_columns(f, chunks[1]);
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

            render_file_navigator(f, chunks[0]);
            render_tag_columns(f, chunks[1]);
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
            .fg(Color::Blue)
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
            .fg(Color::Blue)
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

fn render_file_navigator<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    render_column_block(f, chunk, "File Navigator");
}

fn render_tag_columns<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
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

    render_column_block(f, chunks[0], "Current Tags");
    render_column_block(f, chunks[1], "New Tags");
}

fn render_column_block<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    title: &str
) {
    let container = CustomBorder::new()
        .title(title.to_string());

    f.render_widget(container, chunk);
}
