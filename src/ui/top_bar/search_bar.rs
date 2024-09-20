use tui_realm_stdlib::Input;
use tuirealm::{
    props::{Borders, Style, TextModifiers},
    AttrValue, MockComponent,
};

#[derive(MockComponent)]
pub(super) struct SearchBar {
    component: Input,
}

impl SearchBar {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        let mut input = Input::default()
            .borders(Borders::default().sides(tuirealm::props::BorderSides::all()))
            .placeholder(
                "Search...",
                Style::default().add_modifier(TextModifiers::ITALIC),
            )
            .input_type(tuirealm::props::InputType::Text);

        input.attr(tuirealm::Attribute::Scroll, AttrValue::Flag(true));
        input.attr(tuirealm::Attribute::ScrollStep, AttrValue::Number(1));

        SearchBar { component: input }
    }
}
