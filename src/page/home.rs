use crossterm::event as cse;

use ratatui::Frame;
use ratatui::layout::{Rect, Layout, Constraint};
use ratatui::style::{Style, Modifier};
use ratatui::widgets::{List, ListState, ListItem};

use crate::events::{AppEventDispatcher, AppAction, AppEvent};
use super::{LoadablePage, Page, PageHandleEvent};

const SELECTED_STYLE: Style = Style::new()
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::UNDERLINED);

#[derive(Debug)]
pub struct MenuItem {
    name: String,
    action: fn() -> AppEvent,
}

#[derive(Debug)]
pub struct MenuPage {
    pub menu_items: Vec<MenuItem>,
    pub menu_max_len: u16,
    pub state: ListState,
}

impl MenuPage {
    pub fn new(menu_items: Vec<MenuItem>) -> Self {
        assert!(!menu_items.is_empty());

        let mut state = ListState::default();
        state.select(Some(0));

        let menu_max_len = menu_items.iter()
            .map(|item| item.name.len())
            .max()
            .unwrap();

        MenuPage {
            menu_items,
            menu_max_len: menu_max_len as u16,
            state
        }
    }
}

impl Default for MenuPage {
    fn default() -> Self {
        let items = vec![
            MenuItem {
                name: "Quick Game".to_string(),
                action: || {
                    AppEvent::App(AppAction::GoTo(LoadablePage::GamePage))
                }
            },
            MenuItem {
                name: "Quit".to_string(),
                action: || {
                    AppEvent::App(AppAction::Exit)
                }
            },
        ];

        MenuPage::new(items)
    }
}

impl Page for MenuPage {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let [_, content, _] = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Length(self.menu_max_len + 3),
            Constraint::Min(1),
        ]).areas::<3>(rect);

        let list = List::new(&self.menu_items)
            .highlight_style(SELECTED_STYLE);

        frame.render_stateful_widget(list, content, &mut self.state);
    }

    fn handle_event(&mut self, app_events: AppEventDispatcher, event: &AppEvent) -> PageHandleEvent {
        match event {
            AppEvent::Crossterm(cse) => {
                self.handle_cse_event(app_events, cse)
            },
            AppEvent::App(_) => {
                PageHandleEvent::None
            }
        }
    }
    // fn action(&mut self) -> AppAction {
    //     std::mem::replace(&mut self.action_todo, AppAction::None)
    // }

    fn page_title(&self) -> &str {
        "Main Page"
    }
}

impl MenuPage {
    fn handle_page_action(&mut self, event_dispatcher: AppEventDispatcher) {
        let selected = self.state.selected().unwrap();
        let item = &self.menu_items[selected];

        event_dispatcher.dispatch((item.action)())
    }

    fn handle_cse_event(&mut self, event_dispatcher: AppEventDispatcher, event: &cse::Event) -> PageHandleEvent {
        match event {
            cse::Event::Key(key_event) if key_event.kind == cse::KeyEventKind::Press => {
                return self.handle_cse_key_event(event_dispatcher, key_event);
            },
            _ => {}
        }

        PageHandleEvent::None
    }


    fn handle_cse_key_event(&mut self, event_dispatcher: AppEventDispatcher, event: &cse::KeyEvent) -> PageHandleEvent {
        match event.code {
            cse::KeyCode::Char('k') | cse::KeyCode::Up => {
                self.state.select_previous();
            },
            cse::KeyCode::Char('j') | cse::KeyCode::Down => {
                self.state.select_next();
            },
            cse::KeyCode::Enter => {
                self.handle_page_action(event_dispatcher)
            },
            _ => {},
        }

        PageHandleEvent::None
    }
}

impl From<&MenuItem> for ListItem<'_> {
    fn from(item: &MenuItem) -> Self {
        ListItem::new(item.name.clone())
    }
}
