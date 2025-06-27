mod edit;
mod handle_event;
mod model;
mod tui;
mod update;
mod view;

use crate::model::{Model, RunningState};
use crate::view::view;
use handle_event::handle_event;
use update::update;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    file_name: Option<String>,
}

fn main() -> color_eyre::Result<()> {
    let args = Args::parse();

    tui::install_panic_hook();

    let mut terminal = tui::init_terminal()?;
    let mut model = Model {
        file_name: args.file_name,
        ..Default::default()
    };

    while model.running_state != RunningState::Done {
        terminal.draw(|f| view(&mut model, f))?;

        let mut current_msg = handle_event(&model)?;

        while let Some(msg) = current_msg {
            current_msg = update(&mut model, msg);
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
