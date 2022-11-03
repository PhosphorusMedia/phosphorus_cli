use tui_realm_stdlib::Table;
use tuirealm::{
    props::{Color, TableBuilder, TextModifiers, TextSpan},
    Component, MockComponent, NoUserEvent,
};

use crate::ui::AppMsg;

#[derive(MockComponent)]
pub struct HelpWindow {
    component: Table,
}

impl HelpWindow {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Default for HelpWindow {
    fn default() -> Self {
        let list = vec![
            ["ESC ESC", "Terminates the application"],
            ["Esc", "Closes any secondary window open"],
            ["Ctrl + H", "Shows the help window"],
            ["ENTER", "Pressed over a playlist, shows its songs"],
            ["+", "Pressed over a song, pushes it in the queue"],
            ["-", "Pressed over a song in the queue, removes it"]
        ];
        let mut builder = TableBuilder::default();
        for row in &list.as_slice()[0..&list.len() - 1] {
            for item in row {
                builder.add_col(TextSpan::new(item).italic());
            }
            builder.add_row();
        }
        for item in list.get(list.len() - 1).unwrap() {
            builder.add_col(TextSpan::new(item).italic());
        }

        Self {
            component: Table::default()
                .highlighted_color(Color::LightYellow)
                .scroll(true)
                .table(builder.build())
                .headers(&["Key combo", "Effect"])
                .highlighted_str("➤ ")
                .row_height(1)
                .widths(&[25, 75])
                .modifiers(TextModifiers::BOLD | TextModifiers::UNDERLINED),
        }
    }
}

impl Component<AppMsg, NoUserEvent> for HelpWindow {
    fn on(&mut self, _ev: tuirealm::Event<NoUserEvent>) -> Option<AppMsg> {
        Some(AppMsg::None)
    }
}
