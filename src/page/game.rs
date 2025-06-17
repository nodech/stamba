use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    KeyModifiers,
};

use std::time::Instant;

use ratatui::Frame;
use ratatui::text::{Text, Line, Span};
use ratatui::layout::{Rect, Layout, Constraint};
use ratatui::style::{Style, Modifier, Stylize, Color};
use ratatui::widgets::{Paragraph, Wrap, Block, Borders};

use crate::app::AppAction;
use super::Page;

const DEFAULT_TEXT: &str = include_str!("../../data/example.txt");

const STYLE_ACTIVE_WORD: Style = Style::new()
    .add_modifier(Modifier::UNDERLINED);

const STYLE_CURSOR: Style = STYLE_ACTIVE_WORD
    .bg(Color::Gray)
    .fg(Color::Black);

const STYLE_BAD_CURSOR: Style = STYLE_ACTIVE_WORD
    .bg(Color::Gray)
    .fg(Color::Red);

const STYLE_WRONG_CHARS: Style = STYLE_ACTIVE_WORD
    .fg(Color::Red);

const STYLE_CORRECT_CHARS: Style = STYLE_ACTIVE_WORD
    .fg(Color::Green);

const STYLE_INPUT_PREFIX: Style = Style::new()
    .fg(Color::Cyan);

const STYLE_INPUT_PREFIX_ERROR: Style = STYLE_INPUT_PREFIX
    .fg(Color::Red);

const STYLE_INPUT: Style = Style::new()
    .add_modifier(Modifier::UNDERLINED)
    .fg(Color::Gray);

#[derive(Debug)]
pub struct GamePage {
    pub text_state: GameText,

    pub input_text: String,
    pub has_error: bool,
    pub error_at_char: usize,
    pub current_word: usize,

    pub started: bool,
    pub done: bool,

    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
}

#[derive(Debug)]
pub struct GameText {
    // pub raw: String,
    pub words: Vec<String>,
}

impl GamePage {
    pub fn new(text: String) -> Self {
        let text = text.trim().to_string();

        let words: Vec<String> = text.split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let game_text = GameText {
            // raw: text.to_string(),
            words,
        };

        GamePage {
            text_state: game_text,
            input_text: String::from(""),

            has_error: false,
            error_at_char: 0,
            current_word: 0,

            done: false,
            started: false,

            start_time: None,
            end_time: None,
        }
    }

    pub fn default() -> Self {
        GamePage::new(DEFAULT_TEXT.to_string())
    }

    fn verify_word(&mut self) {
        let current_word = &self.text_state.words[self.current_word];
        let has_next = self.current_word + 1 != self.text_state.words.len();

        if has_next && self.input_text == format!("{current_word} ") {
            self.current_word += 1;
            self.input_text = String::from("");
            self.has_error = false;
        }

        if !has_next && &self.input_text == current_word {
            self.current_word += 1;
            self.input_text = String::from("");
            self.has_error = false;
            self.done = true;
            self.end_time = Some(Instant::now());
        }

        let iter = self.input_text.chars()
            .zip(current_word.chars())
            .enumerate();

        self.has_error = false;

        for (index, (c1, c2)) in iter {
            if c1 != c2 {
                self.has_error = true;
                self.error_at_char = index;
                break;
            }
        }
    }

    fn handle_key(&mut self, event_key: &KeyEvent) {
        if self.done {
            return;
        }

        if !self.started {
            if let KeyCode::Enter = event_key.code {
                self.started = true;
                self.start_time = Some(Instant::now());
            }
            return;
        }

        match event_key.code {
            KeyCode::Backspace => {
                self.input_text.pop();
                self.verify_word();
            },
            KeyCode::Char(c) => {
                if event_key.modifiers.contains(KeyModifiers::CONTROL) {
                    match c {
                        'w' => {
                            self.input_text = String::from("");
                            self.verify_word();
                            return;
                        },
                        _ => {},
                    }
                }

                self.input_text.push(c);
                self.verify_word();
            },
            _ => {},
        }
    }

    fn format_current_word<'a>(&'a self, word: &'a str) -> Vec<Span<'a>> {
        if !self.has_error {
            let split_min = self.input_text.len().min(word.len());
            let (typed, left) = word.split_at(split_min);
            let mut first: &str = left;
            let mut rest: &str = "";

            if !left.is_empty() {
                (first, rest) = left.split_at(1);
            }

            vec![
                Span::styled(typed, STYLE_CORRECT_CHARS),
                Span::styled(first, STYLE_CURSOR),
                Span::styled(rest, STYLE_ACTIVE_WORD),
                Span::from(" "),
            ]
        } else {
            let after_error = self.input_text.len().min(word.len())
                - self.error_at_char;
            let (typed, badrest) = word.split_at(self.error_at_char);
            let (bad, left) = badrest.split_at(after_error);
            let mut first: &str = left;
            let mut rest : &str = "";

            if !left.is_empty() {
                (first, rest) = left.split_at(1);
            }

            vec![
                Span::styled(typed, STYLE_CORRECT_CHARS),
                Span::styled(bad, STYLE_WRONG_CHARS),
                Span::styled(first, STYLE_BAD_CURSOR),
                Span::styled(rest, STYLE_ACTIVE_WORD),
                Span::from(" "),
            ]
        }
    }
}

impl Page for GamePage {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let words: Vec<Span> = self.text_state.words.iter()
            .enumerate()
            .flat_map(|(index, word)| {
                if index < self.current_word {
                    vec![format!("{word} ").fg(Color::Green)]
                } else if index == self.current_word {
                    self.format_current_word(word)
                } else {
                    vec![format!("{word} ").into()]
                }
            })
            .collect();

        let line = Line::from(words);
        let text = Text::from(line);
        let par = Paragraph::new(text)
            .wrap(Wrap { trim: true });

        let lines = par.line_count(rect.width) as u16 + 1;

        let [text_area, input_area] = Layout::vertical([
            Constraint::Length(lines),
            Constraint::Length(2),
        ]).areas::<2>(rect);


        frame.render_widget(par, text_area);

        let mut block = Block::new().borders(Borders::TOP);

        if !self.done {
            let text = Text::from(Line::from(vec![
                Span::styled("> ", if self.has_error {
                        STYLE_INPUT_PREFIX_ERROR
                    } else {
                        STYLE_INPUT_PREFIX
                    }),
                Span::styled(
                    self.input_text.as_str(),
                    STYLE_INPUT
                )
            ]));

            if !self.started {
                block = block.title("Press Enter to start.");
            }

            let input = Paragraph::new(text)
                .block(block);

            frame.render_widget(input, input_area);
        } else {
            let duration = self.end_time.unwrap()
                .duration_since(self.start_time.unwrap());
            let words = self.text_state.words.len();
            let minutes = duration.as_secs_f64() / 60.0;
            let words_per_minute = (words as f64 / minutes) as u32;

            let stats = Paragraph::new(format!("WPM: {words_per_minute}"))
                .block(block);

            frame.render_widget(stats, input_area);
        }
    }

    fn handle_events(&mut self, event: &Event) {
        match event {
            Event::Key(event_key) if event.is_key_press() => {
                self.handle_key(event_key);
            },
            _ => {},
        }
    }

    fn action(&mut self) -> AppAction {
        AppAction::None
    }

    fn page_title(&self) -> &str {
        "Game"
    }
}
