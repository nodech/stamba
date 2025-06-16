use std::io;
use super::Page;

use ratatui::Frame;
use ratatui::layout::{Rect, Layout, Constraint};
use ratatui::style::{Style, Modifier};
use ratatui::widgets::{List, ListState, ListItem};

const SELECTED_STYLE: Style = Style::new()
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::UNDERLINED);

#[derive(Debug, Default)]
pub struct MenuItem {
    name: String,
}

#[derive(Debug)]
pub struct MenuPage {
    pub menu_items: Vec<MenuItem>,
    pub menu_max_len: u16,
    pub selected: ListState,
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
            selected: state
        }
    }
}

impl Default for MenuPage {
    fn default() -> Self {
        let items = vec![
            MenuItem {
                name: "Quick Game".to_string()
            },
            MenuItem {
                name: "Quit".to_string()
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

        frame.render_stateful_widget(list, content, &mut self.selected);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn page_title(&self) -> &str {
        "Main Page"
    }
}

impl From<&MenuItem> for ListItem<'_> {
    fn from(item: &MenuItem) -> Self {
        ListItem::new(item.name.clone())
    }
}
