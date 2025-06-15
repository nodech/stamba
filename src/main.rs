use std::io;

use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
};

use ratatui::{
    Frame,
    DefaultTerminal,
    layout::{Layout, Constraint, Rect},
    style::{Style, Color, Modifier},
    widgets::Paragraph,
};

mod page;

use page::Page;
use page::MainPage;

const APP_NAME: &str = "Terminal Velocity";

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::with_debug().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
struct App {
    frame: u32,
    debug: bool,
    exit: bool,

    pages: Vec<Box<dyn Page>>
}

impl Default for App {
    fn default() -> Self {
        App {
            exit: false,
            debug: false,
            frame: 0,

            pages: vec![Box::new(MainPage{})]
        }
    }
}

impl App {
    fn with_debug() -> Self {
        let mut app = App::default();
        app.debug = true;
        app
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // Main Loop
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                // KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                KeyCode::Char('c') => {
                    if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                        self.exit = true;
                    }
                },
                _ => {}
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.frame = self.frame.wrapping_add(1);

        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ]);

        let areas = vertical.areas::<3>(frame.area());
        let header = areas[0];

        self.draw_header(frame, header);
    }

    fn draw_header(&self, frame: &mut Frame, header: Rect) {
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
        let page_title_text = self.pages.last().unwrap().page_title();
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
}
