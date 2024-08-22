use std::io::{self, Stdout};

use ratatui::{
  crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  },
  prelude::CrosstermBackend,
  Terminal,
};

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
  execute!(io::stdout(), EnterAlternateScreen)?;
  enable_raw_mode()?;
  set_panic_hook();
  Terminal::new(CrosstermBackend::new(io::stdout()))
}

fn set_panic_hook() {
  let hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
    let _ = restore(); // ignore any errors as we are already failing
    hook(panic_info);
  }));
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
  execute!(io::stdout(), LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(())
}
