use std::fmt::Debug;

use ratatui::Frame;
use ratatui::layout::Rect;

pub mod home;
pub use home::MainPage;

pub trait Page: Debug {
    fn page_title(&self) -> &str;
    fn draw(&mut self, frame: &mut Frame, rect: Rect);
}
