use std::io;
use clap::{Parser, ValueEnum};

mod page;
mod app;

#[derive(Debug, ValueEnum, Clone)]
enum LoadablePages {
    QuickGame,
    Home,
}

#[derive(Parser, Debug)]
struct Args {
    /// Show frame count
    #[arg(short, long)]
    debug: bool,

    /// Default page to load
    #[arg(short, long, default_value_t = LoadablePages::Home,
           value_enum, help = "The page to load on startup")]
    page: LoadablePages,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut pages: Vec<Box<dyn page::Page>> = vec![
        Box::new(page::MenuPage::default())
    ];

    if let LoadablePages::QuickGame = args.page {
        pages.push(Box::new(page::GamePage::new()));
    }

    let mut app = app::App {
        debug: args.debug,
        pages,
        ..Default::default()
    };

    let mut terminal = ratatui::init();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
