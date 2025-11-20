use crate::tui::app::{App, Tab};
use ratatui::{
    layout::Alignment,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, LineGauge, Padding, Paragraph, Widget},
};

impl App {
    pub fn top_left(&self) -> impl Widget {
        let border = &self.config.colors.border;
        let paused = &self.config.colors.paused;
        let timestamp = &self.config.colors.timestamp;

        Paragraph::new(vec![
            Line::from(vec![Span::styled(
                if self.audio.is_empty() {
                    String::new()
                } else {
                    format!(
                        "{:.0}:{:02.0}/{}",
                        self.audio.sink_pos() / 60, // Minutes
                        self.audio.sink_pos() % 60, // Seconds
                        // Seperate function since the display could be None
                        self.data.display_duration_display() // Total time
                    )
                },
                Style::default().fg(self.get_color(timestamp)),
            )]),
            Line::from(vec![Span::styled(
                format!(
                    "{}",
                    match self.audio.is_empty() {
                        true => "stopped",
                        false => match self.audio.paused {
                            true => "paused",
                            false => "playing",
                        },
                    }
                ),
                Style::default().fg(self.get_color(paused)),
            )]),
        ])
        .block(
            Block::new()
                .borders(Borders::TOP | Borders::BOTTOM | Borders::LEFT)
                .border_style(Style::default().fg(self.get_color(border)))
                .border_type(BorderType::Rounded)
                .padding(Padding::horizontal(1)),
        )
        .alignment(Alignment::Left)
    }

    pub fn top_center(&self) -> impl Widget {
        let album = &self.config.colors.album;
        let artist = &self.config.colors.artist;
        let border = &self.config.colors.border;
        let tab_selected = &self.config.colors.tab_selected;
        let tab_unselected = &self.config.colors.tab_unselected;
        let title = &self.config.colors.title;
        let track_num = &self.config.colors.track_num;
        let year = &self.config.colors.year;

        Paragraph::new(if self.audio.is_empty() {
            vec![Line::from("")]
        } else {
            vec![
                Line::from(vec![
                    Span::styled(
                        format!("{}", self.data.display_artist()),
                        Style::default().fg(self.get_color(artist)),
                    ),
                    Span::from(" "),
                    Span::styled(
                        format!("{}", self.data.display_title()),
                        Style::default().fg(self.get_color(title)),
                    ),
                ]),
                Line::from(vec![
                    Span::styled(
                        format!("{}", self.data.display_album()),
                        Style::default().fg(self.get_color(album)),
                    ),
                    Span::from(" "),
                    Span::styled(
                        format!("{}", self.data.display_year()),
                        Style::default().fg(self.get_color(year)),
                    ),
                    Span::from(" "),
                    Span::styled(
                        format!("{}", self.data.display_track_number()),
                        Style::default().fg(self.get_color(track_num)),
                    ),
                ]),
            ]
        })
        .block(
            Block::new()
                .borders(Borders::TOP | Borders::BOTTOM)
                .border_style(Style::default().fg(self.get_color(border)))
                .border_type(BorderType::Rounded)
                .title_bottom(
                    Line::from(vec![
                        Span::styled("┤", self.get_color(border)),
                        Span::styled(
                            " 1 ",
                            match self.tab {
                                Tab::Browser => Style::default().fg(self.get_color(tab_selected)),
                                _ => Style::default().fg(self.get_color(tab_unselected)),
                            },
                        ),
                        Span::styled(
                            " 2 ",
                            match self.tab {
                                Tab::Playlist => Style::default().fg(self.get_color(tab_selected)),
                                _ => Style::default().fg(self.get_color(tab_unselected)),
                            },
                        ),
                        Span::styled("├", self.get_color(border)),
                    ])
                    .centered(),
                ),
        )
        .alignment(Alignment::Center)
    }

    pub fn top_right(&self) -> impl Widget {
        let border = &self.config.colors.border;
        let volume = &self.config.colors.volume;

        Paragraph::new(vec![Line::from(vec![Span::styled(
            format!("{}%", self.audio.vol),
            Style::default().fg(self.get_color(volume)),
        )])])
        .block(
            Block::new()
                .borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT)
                .border_style(Style::default().fg(self.get_color(border)))
                .border_type(BorderType::Rounded)
                .padding(Padding::horizontal(1)),
        )
        .alignment(Alignment::Right)
    }

    pub fn progress_bar(&self) -> impl Widget {
        let seekbar_filled = &self.config.colors.seekbar_filled;
        let seekbar_unfilled = &self.config.colors.seekbar_unfilled;

        LineGauge::default()
            .block(Block::new())
            .label("")
            .ratio({
                if self.audio.is_empty() {
                    0.
                } else {
                    (self.audio.sink_pos_millis() as f64
                        / (self
                            .data
                            .duration_as_secs
                            .expect("This shouldn't fail if the file is loaded correctly")
                            * 1000.0))
                        .clamp(0., 1.)
                }
            })
            .filled_style(Style::default().fg(self.get_color(&seekbar_filled)))
            .unfilled_style(Style::default().fg(self.get_color(&seekbar_unfilled)))
    }
}
