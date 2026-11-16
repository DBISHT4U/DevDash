use crate::git_info::GitInfo;
use crate::system::{self, SystemStats};
use crate::widgets;
use chrono::Local;
use ratatui::{prelude::*, widgets::*};

pub struct DashboardState {
    pub sys: SystemStats,
    pub git: GitInfo,
    pub active_tab: usize,
    pub tick_count: u64,
}

impl DashboardState {
    pub fn new() -> Self {
        Self {
            sys: SystemStats::collect(),
            git: GitInfo::collect(),
            active_tab: 0,
            tick_count: 0,
        }
    }

    pub fn next_tab(&mut self) {
        self.active_tab = (self.active_tab + 1) % 2;
    }

    pub fn prev_tab(&mut self) {
        if self.active_tab == 0 {
            self.active_tab = 1;
        } else {
            self.active_tab -= 1;
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        self.sys.refresh();
    }

    pub fn refresh(&mut self) {
        self.sys = SystemStats::collect();
        self.git = GitInfo::collect();
    }
}

pub fn render(f: &mut Frame, state: &mut DashboardState) {
    let area = f.area();

    // Main layout: header + body + footer
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header
            Constraint::Min(10),   // body
            Constraint::Length(1), // footer
        ])
        .split(area);

    render_header(f, main_layout[0], state);

    match state.active_tab {
        0 => render_system_tab(f, main_layout[1], state),
        1 => render_git_tab(f, main_layout[1], state),
        _ => {}
    }

    render_footer(f, main_layout[2]);
}

fn render_header(f: &mut Frame, area: Rect, state: &DashboardState) {
    let now = Local::now().format("%H:%M:%S  %a %d %b %Y").to_string();
    let username = whoami::username();

    let tabs = vec!["⚡ System", "🔀 Git"];
    let tab_titles: Vec<Line> = tabs.iter().map(|t| Line::from(*t)).collect();

    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30),
            Constraint::Min(20),
            Constraint::Length(28),
        ])
        .split(area);

    // Title
    let title = Paragraph::new(Line::from(vec![
        Span::styled(" 💻 DevDash ", Style::default().fg(Color::Cyan).bold()),
        Span::styled("v0.1.0", Style::default().fg(Color::DarkGray)),
    ]))
    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(title, header_layout[0]);

    // Tabs
    let tabs_widget = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(Color::DarkGray)))
        .select(state.active_tab)
        .style(Style::default().fg(Color::DarkGray))
        .highlight_style(Style::default().fg(Color::Cyan).bold())
        .divider("|");
    f.render_widget(tabs_widget, header_layout[1]);

    // Clock + user
    let info = Paragraph::new(Line::from(vec![
        Span::styled(format!("{} ", username), Style::default().fg(Color::Green)),
        Span::styled(now, Style::default().fg(Color::DarkGray)),
    ]))
    .alignment(Alignment::Right)
    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(info, header_layout[2]);
}

fn render_system_tab(f: &mut Frame, area: Rect, state: &DashboardState) {
    // 3 rows layout
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // System info bar
            Constraint::Min(8),    // CPU + Memory + Disk
            Constraint::Length(12), // Processes
        ])
        .split(area);

    // === Row 1: System info ===
    render_sys_info_bar(f, rows[0], state);

    // === Row 2: CPU + Memory + Disk ===
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(rows[1]);

    render_cpu_panel(f, cols[0], state);
    render_memory_panel(f, cols[1], state);
    render_disk_panel(f, cols[2], state);

    // === Row 3: Top processes ===
    render_processes(f, rows[2], state);
}

fn render_sys_info_bar(f: &mut Frame, area: Rect, state: &DashboardState) {
    let s = &state.sys;
    let uptime = system::format_uptime(s.uptime_secs);
    let net_rx = system::format_bytes(s.net_rx_bytes);
    let net_tx = system::format_bytes(s.net_tx_bytes);

    let battery_str = match (s.battery_percent, s.battery_charging) {
        (Some(pct), Some(true)) => format!("🔋 {:.0}% ⚡", pct),
        (Some(pct), _) => format!("🔋 {:.0}%", pct),
        _ => "🔌 AC".to_string(),
    };

    let info_text = vec![Line::from(vec![
        Span::styled(format!("  🖥  {} ", s.hostname), Style::default().fg(Color::Cyan).bold()),
        Span::styled(format!("│ {} ", s.os_name), Style::default().fg(Color::White)),
        Span::styled(format!("│ 🕐 {} ", uptime), Style::default().fg(Color::Yellow)),
        Span::styled(format!("│ {} ", s.cpu_name), Style::default().fg(Color::Magenta)),
        Span::styled(format!("│ {} cores ", s.cpu_count), Style::default().fg(Color::Green)),
        Span::styled(format!("│ 📥 {} ", net_rx), Style::default().fg(Color::Blue)),
        Span::styled(format!("│ 📤 {} ", net_tx), Style::default().fg(Color::Red)),
        Span::styled(format!("│ {} ", battery_str), Style::default().fg(Color::Yellow)),
        Span::styled(format!("│ 🔄 {} procs", s.processes), Style::default().fg(Color::DarkGray)),
    ])];

    let block = widgets::styled_block("System Info", Color::Cyan);
    let paragraph = Paragraph::new(info_text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn render_cpu_panel(f: &mut Frame, area: Rect, state: &DashboardState) {
    let s = &state.sys;
    let block = widgets::styled_block(
        &format!("CPU  {:.1}%", s.cpu_usage),
        widgets::percent_color(s.cpu_usage as f64),
    );

    let inner = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(2)])
        .split(inner);

    // Gauge
    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(widgets::percent_color(s.cpu_usage as f64))
                .bg(Color::DarkGray),
        )
        .percent(s.cpu_usage.min(100.0) as u16)
        .label(format!("{:.1}%", s.cpu_usage));
    f.render_widget(gauge, layout[0]);

    // Sparkline
    let spark_data: Vec<u64> = s.cpu_history.iter().map(|v| *v as u64).collect();
    let sparkline = Sparkline::default()
        .data(&spark_data)
        .style(Style::default().fg(Color::Cyan))
        .max(100);
    f.render_widget(sparkline, layout[1]);
}

fn render_memory_panel(f: &mut Frame, area: Rect, state: &DashboardState) {
    let s = &state.sys;
    let block = widgets::styled_block(
        &format!("Memory  {:.1}%", s.mem_percent),
        widgets::percent_color(s.mem_percent),
    );

    let inner = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(2),
        ])
        .split(inner);

    // RAM gauge
    let ram_gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(widgets::percent_color(s.mem_percent))
                .bg(Color::DarkGray),
        )
        .percent(s.mem_percent.min(100.0) as u16)
        .label(format!(
            "RAM {:.1}/{:.1} GB",
            s.mem_used_gb, s.mem_total_gb
        ));
    f.render_widget(ram_gauge, layout[0]);

    // Swap
    let swap_pct = if s.swap_total_gb > 0.0 {
        (s.swap_used_gb / s.swap_total_gb * 100.0).min(100.0)
    } else {
        0.0
    };
    let swap_gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(widgets::percent_color(swap_pct))
                .bg(Color::DarkGray),
        )
        .percent(swap_pct as u16)
        .label(format!(
            "Swap {:.1}/{:.1} GB",
            s.swap_used_gb, s.swap_total_gb
        ));
    f.render_widget(swap_gauge, layout[1]);

    // Sparkline
    let spark_data: Vec<u64> = s.mem_history.iter().map(|v| *v as u64).collect();
    let sparkline = Sparkline::default()
        .data(&spark_data)
        .style(Style::default().fg(Color::Magenta))
        .max(100);
    f.render_widget(sparkline, layout[2]);
}

fn render_disk_panel(f: &mut Frame, area: Rect, state: &DashboardState) {
    let block = widgets::styled_block("Disks", Color::Yellow);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let disk_count = state.sys.disks.len().max(1);
    let constraints: Vec<Constraint> = state
        .sys
        .disks
        .iter()
        .map(|_| Constraint::Length(2))
        .collect();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    for (i, disk) in state.sys.disks.iter().enumerate() {
        if i >= layout.len() {
            break;
        }
        let label = if disk.mount.len() > 15 {
            format!("...{}", &disk.mount[disk.mount.len() - 12..])
        } else {
            disk.mount.clone()
        };
        let gauge = Gauge::default()
            .gauge_style(
                Style::default()
                    .fg(widgets::percent_color(disk.percent))
                    .bg(Color::DarkGray),
            )
            .percent(disk.percent.min(100.0) as u16)
            .label(format!(
                "{} {:.0}/{:.0}GB",
                label, disk.used_gb, disk.total_gb
            ));
        f.render_widget(gauge, layout[i]);
    }
}

fn render_processes(f: &mut Frame, area: Rect, state: &DashboardState) {
    let block = widgets::styled_block(
        &format!("Top Processes  ({})", state.sys.processes),
        Color::Red,
    );

    let header = Row::new(vec!["PID", "Name", "CPU %", "Memory"])
        .style(Style::default().fg(Color::Cyan).bold())
        .bottom_margin(0);

    let rows: Vec<Row> = state
        .sys
        .top_processes
        .iter()
        .map(|p| {
            let cpu_color = widgets::percent_color(p.cpu as f64);
            Row::new(vec![
                Cell::from(format!("{}", p.pid)),
                Cell::from(p.name.clone()).style(Style::default().fg(Color::White)),
                Cell::from(format!("{:.1}%", p.cpu)).style(Style::default().fg(cpu_color)),
                Cell::from(format!("{:.0} MB", p.mem_mb)),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Min(20),
            Constraint::Length(10),
            Constraint::Length(12),
        ],
    )
    .header(header)
    .block(block)
    .row_highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(table, area);
}

fn render_git_tab(f: &mut Frame, area: Rect, state: &DashboardState) {
    let g = &state.git;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Git status
            Constraint::Min(8),    // Recent commits
        ])
        .split(area);

    // === Git Status ===
    let status_color = widgets::git_status_color(g.staged, g.modified, g.untracked);
    let block = widgets::styled_block("Git Repository", status_color);

    let branch_icon = if g.branch == "main" || g.branch == "master" {
        "🔀"
    } else {
        "🌿"
    };

    let status_lines = vec![
        Line::from(vec![
            Span::styled("  Branch: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} {} ", branch_icon, g.branch),
                Style::default().fg(Color::Green).bold(),
            ),
            Span::styled(
                format!("│ Commits: {} ", g.commit_count),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(
                format!("│ Status: {} ", g.status_summary()),
                Style::default().fg(status_color),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Remote: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&g.remote_url, Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![
            Span::styled("  Latest: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&g.last_commit, Style::default().fg(Color::White)),
            Span::styled(
                format!("  by {} ({})", g.last_author, g.last_date),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                format!("  ↑{} ahead  ↓{} behind", g.ahead, g.behind),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let git_paragraph = Paragraph::new(status_lines).block(block);
    f.render_widget(git_paragraph, layout[0]);

    // === Recent Commits ===
    let commits_block = widgets::styled_block("Recent Commits", Color::Magenta);

    let header = Row::new(vec!["Hash", "Message", "Author", "Date"])
        .style(Style::default().fg(Color::Cyan).bold());

    let rows: Vec<Row> = g
        .recent_commits
        .iter()
        .map(|c| {
            Row::new(vec![
                Cell::from(c.hash.clone()).style(Style::default().fg(Color::Yellow)),
                Cell::from(c.message.clone()).style(Style::default().fg(Color::White)),
                Cell::from(c.author.clone()).style(Style::default().fg(Color::Green)),
                Cell::from(c.date.clone()).style(Style::default().fg(Color::DarkGray)),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Min(30),
            Constraint::Length(16),
            Constraint::Length(16),
        ],
    )
    .header(header)
    .block(commits_block);

    f.render_widget(table, layout[1]);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" Tab", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" switch  ", Style::default().fg(Color::DarkGray)),
        Span::styled("r", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" refresh  ", Style::default().fg(Color::DarkGray)),
        Span::styled("q", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" quit  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "│ github.com/DBISHT4U/devdash",
            Style::default().fg(Color::DarkGray),
        ),
    ]))
    .style(Style::default().bg(Color::Rgb(20, 20, 30)));
    f.render_widget(footer, area);
}
