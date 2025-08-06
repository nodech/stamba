use std::fmt::Debug;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::events::{AppEvent, AppEventDispatcher};

pub mod home;
pub use home::MenuPage;

pub mod game;
pub use game::GamePage;

#[derive(Debug, Clone)]
pub enum LoadablePage {
    MainMenu,
    GamePage,
}

// pub struct PageInfo {
//     pub page: LoadablePage,
//     pub name: &'static str,
// }

// pub const ALL_PAGES: &[PageInfo] = &[
//     PageInfo {
//         page: LoadablePage::MainMenu,
//         name: "Main Menu"
//     },
// ];

pub fn get_page(page: LoadablePage) -> Box<dyn Page> {
    match page {
        LoadablePage::MainMenu => Box::new(MenuPage::default()),
        LoadablePage::GamePage => Box::new(GamePage::default())
    }
}

pub enum PageHandleEvent {
    None,
    Consume
}


pub trait Page: Debug {
    fn page_title(&self) -> &str;
    fn draw(&mut self, frame: &mut Frame, rect: Rect);
    fn handle_event(&mut self, dispatcher: AppEventDispatcher, event: &AppEvent) -> PageHandleEvent;

    // life cycle methods
    // fn on_load(&mut self);
    // fn on_unload(&mut self);
    // fn on_init(&mut);

}
