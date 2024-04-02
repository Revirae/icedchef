use iced::alignment;
use iced::widget::{
    text, text_input,
    button,
    column, row, container
};
use iced::{
    Alignment, Command, Element, Length
};
use uuid::Uuid;
use crate::persistence::InventoryPersistence;
use crate::errors::{LoadError, SaveError};

#[derive(Debug, Clone)]
pub enum AppMessage {
    Loaded(Result<InventoryPersistence, LoadError>),
    Saved(Result<(), SaveError>),
    InventoryMessage(Vec<Uuid>, InventoryMessage),
}

#[derive(Debug, Clone)]
pub enum InventoryMessage {
    Delete,
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
