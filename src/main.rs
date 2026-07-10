mod chroma;

use std::io::{self, stdout};

use ratatui::{
    Terminal, TerminalOptions, Viewport,
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{self, disable_raw_mode, enable_raw_mode},
    },
    style::{Color, Style},
    widgets::Block,
};

// Runs in alternate screen for fullscreen TUI experience
fn run_fullscreen() -> io::Result<()> {
    ratatui::run(|terminal| -> io::Result<()> {
        let mut should_quit = false;
        while !should_quit {
            terminal.draw(|frame| {
                let area = frame.area();
                // Match background color of terminal by not setting a color
                let block = Block::bordered();
                frame.render_widget(block, area);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.kind {
                    KeyEventKind::Press => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => should_quit = true,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    })?;
    Ok(())
}

// Runs in primary screen for an inline experience
fn run_inline(height_percent: u8) -> io::Result<()> {
    // Get user's terminal size
    let (_width, mut height) = terminal::size()?;
    height = height * u16::from(height_percent) / 100;

    std::panic::set_hook(Box::new(|info| {
        let _ = disable_raw_mode();
        eprintln!("{info}");
    }));

    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(height),
        },
    )?;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            // Match background color of terminal by not setting a color
            let block = Block::bordered();
            frame.render_widget(block, area);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.kind {
                KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                },
                _ => {}
            }
        }
    }

    terminal.clear()?;
    disable_raw_mode()?;
    Ok(())
}

fn main() -> io::Result<()> {
    // HACK: this will be a flag or something at some point to decide between alt or primary screen
    let height: Option<u8> = None;
    match height {
        Some(h) => run_inline(h),
        None => run_fullscreen(),
    }
}
