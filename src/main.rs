use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, List, ListItem},
};

pub mod ui;

use ui::*;

#[derive(Debug, Default)]
pub struct App {
    pub task: TaskList,

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

    fn draw(&mut self, frame: &mut Frame) {
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

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // พื้นที่ซ้าย: 40%
                Constraint::Percentage(70), // พื้นที่ขวา: 60%
            ])
            .split(main_chunks[1]);

        frame.render_widget(&HeaderWidget, main_chunks[0]);
        // frame.render_widget(&MainWidget { task: &self.task }, main_chunks[1]);
        frame.render_widget(&FooterWidget, main_chunks[2]);

        self.render_task_list(frame, chunks[0]);
    }

    fn render_task_list(&mut self, frame: &mut Frame, area: Rect) {
        let task_container = Block::bordered().title("Tasks");

        let mut items: Vec<ListItem> = Vec::new();

        for item in &self.task.items {
            items.push(ListItem::new(Line::from(String::from(&item.label))));
        }

        let task_list = List::new(items)
            .highlight_style(Style::new().bg(Color::Red))
            .block(task_container);

        frame.render_stateful_widget(task_list, area, &mut self.task.state);
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
            KeyCode::Char('j') => self.task.state.select_next(),
            KeyCode::Char('k') => self.task.state.select_previous(),
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
