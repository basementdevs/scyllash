use color_eyre::eyre::{Context, Result};
use ratatui::{
  crossterm::event::{self, Event, KeyEventKind},
  prelude::*,
  widgets::{
    block::{Position, Title},
    Block, Paragraph, Widget,
  },
  Frame,
};
use symbols::border;

use crate::tui::Tui;

#[derive(Debug)]
pub struct App {
  current_keyspace: String,
  exit: bool,
}

impl Default for App {
  fn default() -> Self {
    Self {
      current_keyspace: "system".to_string(),
      exit: false,
    }
  }
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
    match key_event.code {
      event::KeyCode::Char('q') => self.exit(),
      _ => {}
    }
    Ok(())
  }

  fn exit(&mut self) {
    self.exit = true;
  }
}

impl Widget for &App {
  fn render(self, area: Rect, buf: &mut Buffer)
  where
    Self: Sized,
  {
    let title = Title::from(" Scyllash ".bold());
    let instructions = Title::from(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]));
    let block = Block::bordered()
      .title(title.alignment(Alignment::Center))
      .title(
        instructions
          .alignment(Alignment::Center)
          .position(Position::Bottom),
      )
      .border_set(border::THICK);

    let keyspace_text = Text::from(vec![Line::from(vec![
      "Current keyspace: ".into(),
      self.current_keyspace.clone().bold(),
    ])]);

    Paragraph::new(keyspace_text)
      .centered()
      .block(block)
      .render(area, buf);
  }
}
