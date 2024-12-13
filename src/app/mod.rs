use color_eyre::eyre::{Context, Result};
use ratatui::{
  Frame,
  crossterm::event::{self, Event, KeyEventKind},
  prelude::*,
  widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::tui::Tui;

#[derive(Debug, Default)]
pub struct App {
  exit: bool,
}

impl App {
  pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.render_frame(frame))?;
      self.handle_events().wrap_err("handle events failed")?;
    }
    Ok(())
  }

  fn render_frame(&self, frame: &mut Frame) {
    frame.render_widget(self, frame.area());
  }

  fn handle_events(&mut self) -> Result<()> {
    match event::read()? {
      // it's important to check that the event is a key press event as
      // crossterm also emits key release and repeat events on Windows.
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
        .handle_key_event(key_event)
        .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
      _ => Ok(()),
    }?;
    Ok(())
  }

  fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Result<()> {
    #[allow(clippy::single_match)]
    match key_event.code {
      event::KeyCode::Char('q') => self.exit(),
      _ => {}
    }
    Ok(())
  }

  fn exit(&mut self) {
    self.exit = true;
  }

  fn sidebar(area: Rect, buf: &mut Buffer) {
    let layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
      .split(area);

    Paragraph::new("Select your keyspace")
      .block(Block::new().borders(Borders::ALL))
      .render(layout[0], buf);

    Paragraph::new("List of keyspaces will be here")
      .wrap(Wrap { trim: false })
      .block(Block::new().borders(Borders::ALL))
      .render(layout[1], buf);
  }

  fn content(area: Rect, buf: &mut Buffer) {
    let layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
      .split(area);

    Paragraph::new("ScyllaSH 0.0.1")
      .block(Block::new().borders(Borders::ALL))
      .render(layout[0], buf);

    Paragraph::new("REPL will be here")
      .block(Block::new().borders(Borders::ALL))
      .render(layout[1], buf);
  }
}

impl Widget for &App {
  fn render(self, area: Rect, buf: &mut Buffer)
  where
    Self: Sized,
  {
    let [sidebar_area, content_area] =
      Layout::horizontal([Constraint::Length(30), Constraint::Fill(1)]).areas(area);

    App::sidebar(sidebar_area, buf);
    App::content(content_area, buf);
  }
}
