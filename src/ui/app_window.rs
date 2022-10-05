use tui_realm_stdlib::Container;
use tuirealm::{MockComponent, props::Alignment, Component, NoUserEvent, event::{KeyEvent, Key, KeyModifiers}};

use super::AppMsg;

#[derive(MockComponent)]
pub struct AppWindow {
    component: Container
}

impl AppWindow {
    pub fn new() -> Self {
        Self {
            component: Container::default().title("Phosphorus",Alignment::Center)
        }
    }
}

impl Component<AppMsg, NoUserEvent> for AppWindow {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<AppMsg> {
        match ev {
            tuirealm::Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE
            }) => Some(AppMsg::Quit),
            _ => None
        }
    }
}