use std::fmt::Debug;

use crossterm::event::Event;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::app::AppAction;

pub mod home;
pub use home::MenuPage;

pub trait Page: Debug {
    fn page_title(&self) -> &str;
    fn draw(&mut self, frame: &mut Frame, rect: Rect);
    fn handle_events(&mut self, key_event: &Event);
    fn action(&mut self) -> AppAction;
}
