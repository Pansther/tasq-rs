use std::fmt::Debug;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
};
pub struct TaskList {
    pub items: Vec<TaskItem>,
    pub state: ListState,
}

pub struct TaskItem {
    pub label: String,
}

pub struct MainWidget<'a> {
    pub task: &'a TaskList,
}

impl Debug for TaskList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FFF").finish()
    }
}

impl Default for TaskList {
    fn default() -> Self {
        let mut state = ListState::default();

        state.select(Some(0));

        Self {
            state: state,
            items: Vec::from([
                TaskItem {
                    label: String::from("Item 1"),
                },
                TaskItem {
                    label: String::from("Item 2"),
                },
                TaskItem {
                    label: String::from("Item 3"),
                },
                TaskItem {
                    label: String::from("Item 4"),
                },
                TaskItem {
                    label: String::from("Item 5"),
                },
                TaskItem {
                    label: String::from("Item 6"),
                },
                TaskItem {
                    label: String::from("Item 7"),
                },
                TaskItem {
                    label: String::from("Item 8"),
                },
            ]),
        }
    }
}

impl<'a> Widget for &MainWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // พื้นที่ซ้าย: 40%
                Constraint::Percentage(70), // พื้นที่ขวา: 60%
            ])
            .split(area);

        let task_container = Block::bordered().title("Tasks");
        let detail_container = Block::bordered().title("Detail");

        let mut items2: Vec<ListItem> = Vec::new();

        for item in &self.task.items {
            items2.push(ListItem::new(Line::from(String::from(&item.label))));
        }

        List::new(items2)
            .block(task_container)
            .highlight_style(Style::new().bg(Color::Red))
            .render(chunks[0], buf);

        let selected = &self.task.state.selected().unwrap();

        Paragraph::new(String::from(selected.to_string()))
            .block(detail_container)
            .render(chunks[1], buf);
    }
}
