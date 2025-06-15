use super::Page;

use ratatui::Frame;
use ratatui::layout::Rect;

#[derive(Debug)]
pub struct MainPage {
    
}

impl Page for MainPage {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
    }

    fn page_title(&self) -> &str {
        "Main Page"
    }
}
