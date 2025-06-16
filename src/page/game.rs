use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
};

use ratatui::Frame;
use ratatui::layout::{Rect, Layout, Constraint};
use ratatui::style::{Style, Modifier};
use ratatui::widgets::{Paragraph};

use crate::app::AppAction;
use super::Page;

#[derive(Debug)]
pub struct GamePage {
    pub selected_text: String,
    pub input_text: String,
}

impl GamePage {
    pub fn new() -> Self {
        GamePage {
            selected_text: include_str!("../../data/example.txt").to_string(),
            input_text: String::from(""),
        }
    }
}

impl Page for GamePage {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let par = Paragraph::new("Hello...");

        frame.render_widget(par, rect);
    }

    fn handle_events(&mut self, event: &Event) {
        // match event {
        // }
    }

    fn action(&mut self) -> AppAction {
        AppAction::None
    }

    fn page_title(&self) -> &str {
        "Game"
    }
}
