mod action;
mod app;
mod client;
mod editor;
mod error;
mod history;
mod input;
mod request;
mod response;
mod terminal;
mod ui;

use clap::Parser;

use app::App;
use client::HttpClient;
use history::HistoryStore;
use request::{HttpMethod, HttpRequest};
use terminal::setup_terminal;
use ui::draw;

use crate::error::EzcurlError;
use crossterm::event::{self, Event};

#[derive(Debug, Parser)]
#[command(version, author, about, arg_required_else_help = true)]
struct Cli {
    url: String,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    Ok(run().await?)
}

async fn run() -> Result<(), EzcurlError> {
    let args = Cli::parse();
    let url = args.url;

    let mut request = HttpRequest::new(HttpMethod::Get, url);
    request.add_header("User-Agent", "ezcurl/0.1");
    request.add_header("Accept", "text/html");

    let client = HttpClient::new();

    let history_store = HistoryStore::for_current_user()?;
    let mut app = App::new(request, client, history_store);

    let mut terminal = setup_terminal()?;

    while !app.should_quit() {
        terminal.draw(|frame| draw(frame, &app))?;
        let event = event::read()?;

        if let Event::Key(key) = event
            && let Some(action) =
                input::map_key(key, app.mode(), app.focused_panel(), app.leader_pending())
        {
            app.handle_action(action).await;
        }
    }

    terminal::exit_terminal(&mut terminal)?;

    Ok(())
}
