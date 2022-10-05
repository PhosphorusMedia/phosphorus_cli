use tuirealm::{PollStrategy, Update, Attribute, AttrValue};
use ui::Model;

use crate::ui::Id;

mod ui;

fn main() {
    // Setup model
    let mut model = Model::default();
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
            },
            Err(err) => {
                assert!(model
                    .app
                    .attr(
                        &Id::Label,
                        Attribute::Text,
                        AttrValue::String(format!("Application error: {}", err)),
                    )
                    .is_ok());
            },
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
    let _ = model.terminal.clear_screen();
}

