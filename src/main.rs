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
fn run_inline() -> io::Result<()> {
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
    let height: Option<u8> = Some(0);
    match height {
        Some(h) if 0 < h && h < 100 => run_inline(),
        _ => run_fullscreen(),
    }
}
