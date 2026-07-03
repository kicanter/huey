mod chroma;

use std::io::{self, stdout};

use ratatui::{
    Terminal, TerminalOptions, Viewport,
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode},
    },
    style::{Color, Style},
    widgets::Block,
};

const HEIGHT: u16 = 8;

fn main() -> io::Result<()> {
    std::panic::set_hook(Box::new(|info| {
        let _ = disable_raw_mode();
        eprintln!("{info}");
    }));

    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(HEIGHT),
        },
    )?;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let swatch_color = Color::Rgb(255, 136, 0);
            let block = Block::bordered().style(Style::default().bg(swatch_color));
            frame.render_widget(block, area);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    terminal.clear()?;
    disable_raw_mode()?;
    Ok(())
}
