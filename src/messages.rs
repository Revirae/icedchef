#![allow(unused)]
use iced::{alignment, Theme};
use iced::widget::{
    container, text, text_input, Column, Row, Text, TextInput
};
use iced::{
    Element, Length
};
use iced_aw::native::{tabs, tab_bar};
use iced_aw::{number_input, NumberInput};
use crate::persistence::Persistence;
use crate::errors::{LoadError, SaveError};
use crate::states::Tab;

#[derive(Debug, Clone)]
pub enum AppMessage {
    Loaded(Result<Persistence, LoadError>),
    Saved(Result<(), SaveError>),
    SelectTab(Tab),
    FormMessage(FormMessage),
}

#[derive(Debug, Clone)]
pub enum FormMessage {
    NameChanged(String),
    BrandChanged(String),
    CostChanged(f64),
    WeightChanged(f64),
    VolumeChanged(f64),
}

