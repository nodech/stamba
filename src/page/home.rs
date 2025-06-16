use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
};

use ratatui::Frame;
use ratatui::layout::{Rect, Layout, Constraint};
use ratatui::style::{Style, Modifier};
use ratatui::widgets::{List, ListState, ListItem};

use crate::app::AppAction;
use super::Page;

const SELECTED_STYLE: Style = Style::new()
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::UNDERLINED);

#[derive(Debug)]
pub struct MenuItem {
    name: String,
    action: fn() -> AppAction,
}

#[derive(Debug)]
pub struct MenuPage {
    pub menu_items: Vec<MenuItem>,
    pub menu_max_len: u16,
    pub state: ListState,

    action_todo: AppAction
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
            state,
            action_todo: AppAction::None
        }
    }
}

impl Default for MenuPage {
    fn default() -> Self {
        let items = vec![
            MenuItem {
                name: "Quick Game".to_string(),
                action: || -> AppAction {
                    AppAction::None
                }
            },
            MenuItem {
                name: "Quit".to_string(),
                action: || {
                    AppAction::Exit
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

    fn handle_events(&mut self, event: &Event) {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_event(key_event);
            },
            _ => {}
        }
    }

    fn action(&mut self) -> AppAction {
        std::mem::replace(&mut self.action_todo, AppAction::None)
    }

    fn page_title(&self) -> &str {
        "Main Page"
    }
}

impl MenuPage {
    fn goto_page(&mut self) {
        let selected = self.state.selected().unwrap();
        let item = &self.menu_items[selected];

        self.action_todo = (item.action)();
    }

    fn handle_event(&mut self, event: &KeyEvent) {
        match event.code {
            KeyCode::Char('k') | KeyCode::Up => {
                self.state.select_previous();
            },
            KeyCode::Char('j') | KeyCode::Down => {
                self.state.select_next();
            },
            KeyCode::Enter => {
                self.goto_page();
            },
            _ => {},
        }
    }
}

impl From<&MenuItem> for ListItem<'_> {
    fn from(item: &MenuItem) -> Self {
        ListItem::new(item.name.clone())
    }
}
