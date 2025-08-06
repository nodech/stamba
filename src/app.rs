use std::io;

use crossterm::event as cse;

use ratatui::{
    Frame,
    DefaultTerminal,
    layout::{Layout, Constraint, Rect},
    style::{Style, Color, Modifier},
    widgets::{Paragraph, Block, Borders},
};

use crate::events::{AppEventSource, AppEvent, AppAction};

use super::page::{self, Page, PageHandleEvent, LoadablePage};

const APP_NAME: &str = "Stamba";

#[derive(Debug)]
pub struct App {
    pub debug: bool,
    pub frame: u32,
    pub exit: bool,

    pub app_events: Option<AppEventSource>,

    pages: Vec<Box<dyn Page>>,
}

impl Default for App {
    fn default() -> Self {
        App {
            exit: false,
            debug: false,
            frame: 0,

            app_events: None,
            pages: vec![page::get_page(LoadablePage::MainMenu)],
        }
    }
}

// Constructors
impl App {
    pub fn new() -> Self {
        App {
            ..Default::default()
        }
    }

    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

// app loop
impl App {
    pub async fn init(&mut self) -> io::Result<()> {
        self.app_events = Some(AppEventSource::init().await);

        Ok(())
    }

    pub async fn shutdown(&mut self) -> io::Result<()> {
        if let Some(mut events) = self.app_events.take() {
            events.shutdown().await;
        }

        Ok(())
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        assert!(self.app_events.is_some(),
            "AppEvents must be initialized before running the app");

        // Main Loop
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let events = self.app_events.as_mut().unwrap().collect_events().await;

            for event in events.into_iter() {
                self.handle_event(event)?;
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: AppEvent) -> io::Result<()> {
        let dispatcher = self.app_events.as_ref().unwrap().get_dispatcher();
        let page = self.get_active_page_mut();

        if let PageHandleEvent::Consume = page.handle_event(dispatcher, &event) {
            return Ok(())
        }

        match event {
            AppEvent::Crossterm(cse) => {
                self.handle_cse_event(cse)
            },
            AppEvent::App(action) => {
                self.handle_action(action);
                Ok(())
            }
        }
    }

    fn handle_cse_event(&mut self, event: cse::Event) -> io::Result<()> {
        match event {
            cse::Event::Key(key_event) if key_event.kind == cse::KeyEventKind::Press => {
                self.handle_cse_key_event(key_event);
            },
            _ => {}
        }

        Ok(())
    }

    fn handle_cse_key_event(&mut self, key_event: cse::KeyEvent) {
        match key_event.code {
            cse::KeyCode::Char('c') => {
                if key_event.modifiers.contains(cse::KeyModifiers::CONTROL) {
                    self.exit = true;
                }
            },
            cse::KeyCode::Esc => {
                if self.pages.len() > 1 {
                    self.pages.pop();
                }
            },
            _ => {},
        }
    }

    fn handle_action(&mut self, action: AppAction) {
        match action {
            AppAction::GoTo(page) => { self.go_to_page(page) },
            AppAction::Exit => { self.exit = true },
        }
    }
}

// Implement page management.
impl App {
    fn get_active_page(&self) -> &dyn Page {
        self.pages.last().unwrap().as_ref()
    }

    fn get_active_page_mut(&mut self) -> &mut dyn Page {
        self.pages.last_mut().unwrap().as_mut()
    }

    fn go_to_page(&mut self, page_id: LoadablePage) {
        let page = page::get_page(page_id);
        self.pages.push(page);
    }
}

// Implement drawing stuff.
impl App {
    fn draw(&mut self, frame: &mut Frame) {
        self.frame = self.frame.wrapping_add(1);

        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(1),
        ]);

        let [
            header_area,
            page_area,
            footer_area
        ] = vertical.areas::<3>(frame.area());

        self.draw_header(frame, header_area);
        self.draw_page(frame, page_area);
        self.draw_footer(frame, footer_area);
    }

    fn draw_header(&self, frame: &mut Frame, header_space: Rect) {
        let [header, line] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1)
        ]).areas::<2>(header_space);

        let border = Block::new().borders(Borders::BOTTOM);
        frame.render_widget(border, line);

        let constraints = [
            Constraint::Min(APP_NAME.len() as u16 + 2),
            Constraint::Fill(1),
            Constraint::Min(1),
        ];

        let horizontal = Layout::horizontal(constraints)
            .horizontal_margin(1)
            .areas::<3>(header);

        let app_title_style = Style::default().fg(Color::Blue);
        let app_title = Paragraph::new(APP_NAME)
            .style(app_title_style);

        frame.render_widget(app_title, horizontal[0]);

        let page_title_style = Style::default().add_modifier(Modifier::BOLD);
        let page_title_text = self.get_active_page().page_title();
        let page_title = Paragraph::new(page_title_text)
            .style(page_title_style)
            .centered();

        frame.render_widget(page_title, horizontal[1]);

        if self.debug {
            let debug_text = Paragraph::new(self.frame.to_string())
                .right_aligned();

            frame.render_widget(debug_text, horizontal[2]);
        }
    }

    fn draw_footer(&self, frame: &mut Frame, header: Rect) {
        let footer_style = Style::default().fg(Color::Gray);
        let footer_text = Paragraph::new("Ctrl+C to exit, ESC to back")
            .style(footer_style)
            .centered();

        frame.render_widget(footer_text, header);
    }

    fn draw_page(&mut self, frame: &mut Frame, area: Rect) {
        let active_page = self.pages.last_mut().unwrap();
        active_page.draw(frame, area);
    }
}
