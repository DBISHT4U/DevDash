use ratatui::{prelude::*, widgets::*};

/// Create a styled block with title
pub fn styled_block(title: &str, color: Color) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(color))
        .title(format!(" {} ", title))
        .title_style(Style::default().fg(color).bold())
}

/// Create a gauge widget for resource usage
pub fn usage_gauge<'a>(label: &'a str, percent: f64, _color: Color) -> Gauge<'a> {
    let color = match percent as u64 {
        0..=50 => Color::Green,
        51..=75 => Color::Yellow,
        76..=90 => Color::Rgb(255, 165, 0),
        _ => Color::Red,
    };

    Gauge::default()
        .block(Block::default().title(format!(" {} ", label)))
        .gauge_style(Style::default().fg(color).bg(Color::DarkGray))
        .percent(percent.min(100.0) as u16)
        .label(format!("{:.1}%", percent))
}

/// Create a sparkline from data
pub fn spark<'a>(data: &'a [u64], color: Color) -> Sparkline<'a> {
    Sparkline::default()
        .data(data)
        .style(Style::default().fg(color))
        .max(100)
}

/// Color based on percentage
pub fn percent_color(pct: f64) -> Color {
    match pct as u64 {
        0..=50 => Color::Green,
        51..=75 => Color::Yellow,
        76..=90 => Color::Rgb(255, 165, 0),
        _ => Color::Red,
    }
}

/// Color for git status
pub fn git_status_color(staged: usize, modified: usize, untracked: usize) -> Color {
    if staged == 0 && modified == 0 && untracked == 0 {
        Color::Green
    } else if modified > 0 || staged > 0 {
        Color::Yellow
    } else {
        Color::Cyan
    }
}
