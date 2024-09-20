use phosphorus_core::plugin_manager::query::QueryResult;
use tui_realm_stdlib::Table;
use tuirealm::{
    props::{Color, TableBuilder, TextModifiers, TextSpan},
    Component, MockComponent,
};

use crate::ui::{event::UserEvent, AppMsg};

#[derive(MockComponent)]
pub struct ResultsWindow {
    component: Table,
}

impl ResultsWindow {
    pub fn new(result: &QueryResult) -> Self {
        let data = result.data();

        let mut builder = TableBuilder::default();
        if data.len() > 0 {
            for (index, item) in data.iter().enumerate() {
                builder.add_col(TextSpan::new(index.to_string()).italic());
                builder.add_col(TextSpan::new(item.track_name()).italic());
                builder.add_col(TextSpan::new(item.artist_name()).italic());
                builder.add_col(TextSpan::new(item.duration_str()).italic());
                if index < data.len() - 1 {
                    builder.add_row();
                }
            }
        }

        let mut component = Table::default()
            .highlighted_color(Color::LightYellow)
            .scroll(true)
            .title("Search results", tuirealm::props::Alignment::Left)
            .headers(&["#", "Name", "Artist", "Duration"])
            .highlighted_str("➤ ")
            .row_height(1)
            .widths(&[5, 50, 30, 15])
            .modifiers(TextModifiers::BOLD | TextModifiers::UNDERLINED);

        if data.len() > 0 {
            component = component.table(builder.build());
        }

        Self { component }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Component<AppMsg, UserEvent> for ResultsWindow {
    fn on(&mut self, _ev: tuirealm::Event<UserEvent>) -> Option<AppMsg> {
        Some(AppMsg::None)
    }
}
