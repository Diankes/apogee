use color_eyre::Result;
use core::time::Duration;
use crossterm::event;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{RenderDirection, Sparkline};

pub fn run_tui(acceleration_records: &[u64]) -> Result<()> {
    color_eyre::install()?;
    let frame_timeout = Duration::from_secs_f64(1.0 / 60.0); // run at 60 FPS
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                // 1. Define only 2 constraints:
                // - Length(1) for the title line
                // - Fill(1) to let the graph take up all remaining vertical space
                let constraints = [Constraint::Length(1), Constraint::Fill(1)];

                // Create the vertical layout
                let layout = Layout::vertical(constraints).spacing(1);

                // 2. Destructure the layout into exactly 2 areas: top and graph_area
                let [top, graph_area] = frame.area().layout(&layout);

                let title = Line::from_iter([
                    Span::from("Sparkline Widget").bold(),
                    Span::from(" (Press 'q' to quit)"),
                ]);
                frame.render_widget(title.centered(), top);

                // 3. Render the sparkline into the graph_area
                render_acceleration_sparkline(frame, graph_area, acceleration_records);
            })?;
            if event::poll(frame_timeout)? && event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}

/// Render a sparkline with some sample data.
pub fn render_acceleration_sparkline(frame: &mut Frame, area: Rect, accelerations_records: &[u64]) {
    let max_acceleration = accelerations_records.iter().max().unwrap();
    let sparkline = Sparkline::default()
        .data(accelerations_records)
        .max(*max_acceleration)
        .direction(RenderDirection::LeftToRight)
        .style(Color::Cyan);

    frame.render_widget(sparkline, area);
}
