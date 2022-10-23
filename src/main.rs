use core::playlist_manager::PlaylistManager;
use std::ffi::OsString;
use tuirealm::{AttrValue, Attribute, PollStrategy, Update};
use ui::Model;

use crate::ui::Id;

mod ui;

fn main() {
    let config_dir = OsString::from("/home/leonardo/.phosphorus/data/playlists");
    let playlist_manager = match PlaylistManager::load(config_dir) {
        Ok(pm) => pm,
        Err(msg) => {
            eprintln!("An error occured while trying to fetch playlists data");
            eprintln!("{}", msg);
            std::process::exit(1);
        },
    };

    // Setup model
    let mut model = Model::new(playlist_manager);
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
