use phosphorus_core::{
    playlist_manager::{self, PlaylistManager},
    queue::QueueManager,
};
use tuirealm::{AttrValue, Attribute, PollStrategy, Update};
use ui::Model;

use crate::ui::Id;

mod config;
mod downloader;
mod player;
mod ui;

fn main() {
    let paths = match config::config_env() {
        Ok(paths) => paths,
        Err(msg) => {
            eprintln!("Preliminary checks failed");
            eprintln!("The error was: {}", msg);
            std::process::exit(1);
        }
    };

    let mut playlist_manager = match PlaylistManager::load(
        paths.data().clone().into_os_string(),
        paths.playlists().clone().into_os_string(),
    ) {
        Ok(pm) => pm,
        Err(msg) => {
            eprintln!("An error occured while trying to fetch playlists data");
            eprintln!("{}", paths.playlists_as_str());
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    };

    if let Err(msg) = playlist_manager.ensure_basics() {
        eprintln!("An error occured while checking for the existence of basic playlists");
        eprintln!("{}", msg);
        std::process::exit(1);
    }

    let queue_manager = QueueManager::default();

    // Setup model
    let model = Model::new(paths, playlist_manager, queue_manager);
    let mut model = match model {
        Ok(model) => model,
        Err(_) => {
            eprintln!("An error occured while trying to create the querier");
            std::process::exit(1);
        }
    };
    // Enter alternate screen
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();

    while !model.quit {
        // Tick
        match model.app.tick(PollStrategy::Once) {
            Ok(messages) if messages.len() > 0 => {
                // Redraws only if al least one message has been received
                model.redraw = true;
                for msg in messages {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            Err(err) => {
                assert!(model
                    .app
                    .attr(
                        &Id::Label,
                        Attribute::Text,
                        AttrValue::String(format!("Application error: {}", err)),
                    )
                    .is_ok());
            }
            _ => {}
        }

        // Redraw
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }

    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
}
