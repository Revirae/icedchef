#![allow(unused)]
mod states;
mod models;
mod persistence;
mod messages;
mod errors;

use iced::widget::{
    text, text_input,
    button,
    column, row, container
};
use iced::{alignment, executor, keyboard, window};
use iced::widget::{Text, Column};
use iced::{
    Alignment, Command, Element,
    Length, Subscription,
    Application, Settings, Theme
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use states::AppState;
use messages::{loading_message, AppMessage};
use persistence::InventoryPersistence;

fn main() -> iced::Result {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    ChefApp::run(Settings::default())
}

#[derive(Default, Debug)]
enum ChefApp {
    #[default]
    Loading,
    Loaded(AppState)
}

impl iced::Application for ChefApp {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = AppMessage;
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (ChefApp, Command<Self::Message>) {
        (
            ChefApp::Loading,
            Command::perform(
                InventoryPersistence::load(),
                Self::Message::Loaded
            )
        )
    }
    fn title(&self) -> String {
        String::from("Chef")
    }
    fn update(
        &mut self,
        message: Self::Message
    ) -> Command<Self::Message> {
        match self {
            ChefApp::Loading => {
                match message {
                    AppMessage::Loaded(Ok(state)) => {
                        *self = ChefApp::Loaded(AppState
                        {
                            food: state.food,
                            recipes: state.recipes,
                            portions: state.portions,
                            ..AppState::default()
                        });
                    }
                    AppMessage::Loaded(Err(_)) => {
                        *self = ChefApp::Loaded(
                            AppState::default()
                        )
                    }
                    _ => {}
                }
                Command::none()
            }
            ChefApp::Loaded(state) => {
                let mut saved = false;

                let command = match message {
                    AppMessage::Loaded(_) => Command::none(),
                    AppMessage::Saved(_result) => {
                        state.saving = false;
                        saved = true;
                        Command::none()
                    }
                    AppMessage::InventoryMessage(affected, message) => {
                        todo!()
                    }
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        InventoryPersistence {
                            food: state.food.clone(),
                            recipes: state.recipes.clone(),
                            portions: state.portions.clone()
                        }.save(),
                        AppMessage::Saved,
                    )
                } else {
                    Command::none()
                };

                Command::batch(vec![command, save])
            }
        }    
    }
    fn view(&self) -> Element<AppMessage> {
        match self {
            ChefApp::Loading => loading_message(),
            ChefApp::Loaded(AppState
             {
                    food,
                    ..
                }) => {
                    let title = text("Chef")
                        .width(Length::Fill)
                        .size(85)
                        .horizontal_alignment(alignment::Horizontal::Center);

                    //----
                    let content = column![title]
                        .spacing(20)
                        .max_width(800);

                    container(content).padding(40).center_x().into()
                }
        }
    }
}

impl ChefApp {
    fn load() -> Command<AppMessage> {
        Command::perform(
            InventoryPersistence::load(),
            AppMessage::Loaded
        )
    }
}


//-------
