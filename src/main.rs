use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

pub mod ui;
use ui::*;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            // .margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(area);

        frame.render_widget(&HeaderWidget, main_chunks[0]);
        frame.render_widget(&MainWidget, main_chunks[1]);
        frame.render_widget(&FooterWidget, main_chunks[2]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();

    app_result
}
