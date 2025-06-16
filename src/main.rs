use std::io;

mod page;
mod app;

fn main() -> io::Result<()> {
    let debug = std::env::var("DEBUG")
        .is_ok_and(|v| v == "1" || v.to_lowercase() == "true");

    let mut app = app::App {
        debug,
        ..Default::default()
    };

    let mut terminal = ratatui::init();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
