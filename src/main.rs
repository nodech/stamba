use clap::Parser;
use std::io;

mod app;
mod page;
mod events;

#[derive(Parser, Debug)]
struct Args {
    /// Show frame count
    #[arg(short, long)]
    debug: bool,

    /// Default page to load
    #[arg(short, long, default_value = "home",
           value_enum, help = "The page to load on startup")]
    page: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut app = app::App::new()
        .debug(args.debug);

    let mut terminal = ratatui::init();
    app.init().await?;
    app.run(&mut terminal).await?;
    ratatui::restore();

    app.shutdown().await
}
