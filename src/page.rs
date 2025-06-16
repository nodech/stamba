use std::io;
use std::fmt::Debug;

use ratatui::Frame;
use ratatui::layout::Rect;

pub mod home;
pub use home::MenuPage;

pub trait Page: Debug {
    fn page_title(&self) -> &str;
    fn draw(&mut self, frame: &mut Frame, rect: Rect);
    fn handle_events(&mut self) -> io::Result<()>;
}
