use iced::{
    alignment,
    widget::{container, text, Column, Row},
    Element,
    Length
};
use iced_aw::{tab_bar, tabs};

use crate::{
    messages::{AppMessage, FormMessage},
    models::FoodForm,
    states::{AppState, Tab}
};


pub fn loading<'a>()
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

fn food_content<'a>(form: &'a FoodForm) -> Element<'a, AppMessage> {
    let form: Element<AppMessage> = form.view().map(move |message| {
           AppMessage::FormMessage(message) 
    });
    Row::new()
        .push(form)
        .into()
}

fn recipe_content<'a>() -> Element<'a, AppMessage> {
    Column::new().into()
}

pub fn spa<'a>(state: &'a AppState)
    -> Element<'a, AppMessage> {
    let tabs_data: Vec<(Tab, &str)> = vec![
        (Tab::Food, "Alimentos"),
        (Tab::Recipe, "Receitas"),
    ];
    let mut tab_widget = tabs::Tabs::new(AppMessage::SelectTab);
    for (tab, title) in tabs_data.into_iter() {
        tab_widget = tab_widget.push(
            tab.clone(),
            tab_bar::TabLabel::Text(title.to_string()),
            match tab {
                Tab::Food => {
                    food_content(&state.food_form)
                }
                Tab::Recipe => {
                    recipe_content()
                }
            }
        );
    }

    tab_widget = tab_widget.set_active_tab(&state.current_tab);
    Column::new().push(tab_widget).into()
}

