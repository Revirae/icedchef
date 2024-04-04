#![allow(unused)]
mod states;
mod models;
mod persistence;
mod messages;
mod errors;

use iced::advanced::Text;
use iced::widget::{
    text, text_input,
    button,
    column, row, container
};
use iced::{alignment, executor, keyboard, window};
use iced::widget::Column;
use iced::{
    Alignment, Command, Element,
    Length, Subscription,
    Application, Settings, Theme
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use states::{AppState, Tab};
use messages::{loading_message, tabs_view, AppMessage};
use persistence::Persistence;

fn main() -> iced::Result {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    ChefApp::run(Settings::default())
}

#[derive(Default, Debug)]
enum ChefApp {
    #[default]
    Loading,
    Loaded(AppState),
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
                Persistence::load(),
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
                            inventory: state.inventory,
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
                    AppMessage::SelectTab(tab) => {
                        state.current_tab = tab;
                        Command::none()
                    }
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        Persistence {
                            inventory: state.inventory.clone(),
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
            ChefApp::Loaded(AppState{
                    inventory,
                    current_tab,
                    ..
            }) => {
                    tabs_view(current_tab)
                }
        }
    }
}

impl ChefApp {
    fn load() -> Command<AppMessage> {
        Command::perform(
            Persistence::load(),
            AppMessage::Loaded
        )
    }
}

