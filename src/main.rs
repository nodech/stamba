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
    widgets::{Paragraph, Block, Borders},
};

mod page;

use page::Page;
use page::MainPage;

const APP_NAME: &str = "Terminal Velocity";

fn main() -> io::Result<()> {
    let debug = std::env::var("DEBUG")
        .is_ok_and(|v| v == "1" || v.to_lowercase() == "true");

    let mut app = App {
        debug,
        ..Default::default()
    };

    let mut stdout = io::stdout();
    let mut terminal = ratatui::init();
    let app_result = app.run(&mut terminal);
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
            if let KeyCode::Char('c') = key_event.code {
                if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                    self.exit = true;
                }
            }
        }
    }

    fn active_page(&self) -> &dyn Page {
        self.pages.last().unwrap().as_ref()
    }

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
        let page_title_text = self.active_page().page_title();
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
        let footer_text = Paragraph::new("Press Ctrl+C to exit")
            .style(footer_style)
            .centered();

        frame.render_widget(footer_text, header);
    }

    fn draw_page(&mut self, frame: &mut Frame, area: Rect) {
        let active_page = self.pages.last_mut().unwrap();
        active_page.draw(frame, area);
    }
}
