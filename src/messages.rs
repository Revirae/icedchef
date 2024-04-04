use iced::alignment;
use iced::widget::{
    container, text, Column, Text
};
use iced::{
    Element, Length
};
use iced_aw::native::{tabs, tab_bar};
use crate::persistence::Persistence;
use crate::errors::{LoadError, SaveError};
use crate::states::{self, Tab};

#[derive(Debug, Clone)]
pub enum AppMessage {
    Loaded(Result<Persistence, LoadError>),
    Saved(Result<(), SaveError>),
    SelectTab(states::Tab)
}

pub fn loading_message<'a>()
    -> Element<'a, AppMessage> {
        container(
            text("loading...")
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(50),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}

pub fn tabs_view<'a>(this_tab: &Tab)
    -> Element<'a, AppMessage> {
        let tabs_data: Vec<(Tab, &str)> = vec![
            (Tab::Food, "Alimentos"),
            (Tab::Recipe, "Receitas"),
        ];
        let mut tabs = tabs::Tabs::new(AppMessage::SelectTab);
        for (tab, title) in tabs_data.into_iter() {
            tabs = tabs.push(
                tab,
                tab_bar::TabLabel::Text(title.to_string()),
                Text::new(title));
        }

        tabs = tabs.set_active_tab(&this_tab);
        Column::new().push(tabs).into()
    }
