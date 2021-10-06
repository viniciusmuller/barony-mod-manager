use std::{os::unix::prelude::CommandExt, path::PathBuf};

use barony_mod_manager::{
    data::{BaronyMod, SteamWorkshopTag},
    filesystem,
    steam_api::{download_image, get_total_mods, get_workshop_item},
    styling::BaronyModManagerUiStyles,
    widgets::{Message, PickableTag, Sorter},
};
use iced::{
    button, executor, image, pick_list, scrollable, text_input, Align, Application, Button,
    Checkbox, Clipboard, Color, Command, Container, Element, Image, Length, PickList, Row,
    Settings, Subscription, Text, TextInput,
};
use iced_native::{Column, Event, Scrollable};
use reqwest::Client;

fn main() -> iced::Result {
    BaronyModManager::run(Settings {
        exit_on_close_request: false,
        ..Settings::default()
    })
}

/// App state
struct BaronyModManager {
    // Core data
    barony_dir: Option<PathBuf>,
    mods: Option<Vec<BaronyMod>>,
    http_client: Client,
    // Api key input
    steam_api_key: String,
    api_key_input: text_input::State,
    api_key_input_hidden: bool,

    // Mod querying
    mod_search_input: text_input::State,
    query: String,

    // Sorters picklist
    sorter_picklist: pick_list::State<Sorter>,
    selected_sorter: Option<Sorter>,

    tag_picklist: pick_list::State<PickableTag>,
    selected_tag: Option<PickableTag>,

    // Show only installed checkbox
    show_only_installed: bool,

    // Barony dir input
    barony_dir_str: String,
    barony_dir_input: text_input::State,

    // Button
    button_state: button::State,

    mods_scrollable: scrollable::State,

    // Misc
    should_exit: bool,
    error_message: Option<String>,
}

// TODO: Create a workshop items `Tag` picklist

impl Application for BaronyModManager {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn title(&self) -> String {
        String::from("Barony Mod Manager")
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn new(_flags: Self::Flags) -> (BaronyModManager, Command<Message>) {
        let persisted_settings = filesystem::load_persisted_settings();

        let initial_state = BaronyModManager {
            barony_dir: None,
            mods: None,
            http_client: Client::new(),
            // Steam API key
            api_key_input: text_input::State::default(),
            api_key_input_hidden: true,
            steam_api_key: persisted_settings.steam_api_key.unwrap_or_default(),
            // Mod querying
            mod_search_input: text_input::State::default(),
            query: "".to_string(),
            // Pick list
            sorter_picklist: pick_list::State::default(),
            selected_sorter: Some(Sorter::default()),
            show_only_installed: false,

            tag_picklist: pick_list::State::default(),
            selected_tag: Some(PickableTag::default()),

            barony_dir_str: persisted_settings.barony_directory_path.unwrap_or_default(),
            barony_dir_input: text_input::State::default(),

            button_state: button::State::default(),

            mods_scrollable: scrollable::State::default(),

            should_exit: false,
            error_message: None,
        };

        (initial_state, Command::none())
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::ApiKeyInputChanged(new_value) => {
                self.steam_api_key = new_value;
                Command::none()
            }
            Message::ModSearchInputChanged(new_value) => {
                self.query = new_value;
                Command::none()
            }
            Message::ToggleHiddenApiKeyInput(new_value) => {
                self.api_key_input_hidden = new_value;
                Command::none()
            }
            Message::ToggleShowOnlyInstalled(new_value) => {
                self.show_only_installed = new_value;
                Command::none()
            }
            Message::ButtonWasPressed => Command::perform(
                get_total_mods(self.http_client.clone(), self.steam_api_key.clone()),
                |result| match result {
                    Ok(number) => Message::TotalModsNumber(number),
                    Err(message) => Message::ErrorHappened(message),
                },
            ),
            Message::TotalModsNumber(total) => iced::Command::batch((1..=total).map(|n| {
                Command::perform(
                    get_workshop_item(self.http_client.clone(), self.steam_api_key.clone(), n),
                    // Message::Testing,
                    |result| match result {
                        Ok(mod_response) => Message::ModFetched(mod_response),
                        Err(message) => Message::ErrorHappened(message),
                    },
                )
            })),
            Message::SorterSelected(new_sorter) => {
                self.selected_sorter = Some(new_sorter);
                Command::none()
            }
            Message::TagSelected(tag) => {
                self.selected_tag = Some(tag);
                Command::none()
            }
            Message::BaronyDirectoryPathChanged(new_value) => {
                self.barony_dir_str = new_value;
                Command::none()
            }
            Message::EventOccurred(event) => {
                match event {
                    Event::Window(iced_native::window::Event::CloseRequested) => {
                        // Cleanup
                        filesystem::persist_settings(filesystem::SettingsPersistance {
                            barony_directory_path: Some(self.barony_dir_str.clone()),
                            steam_api_key: Some(self.steam_api_key.clone()),
                        });
                        self.should_exit = true;
                    }
                    _ => (),
                }
                Command::none()
            }
            Message::ModFetched(barony_mod) => {
                if let Some(mods) = &mut self.mods {
                    mods.push(barony_mod.clone()) // self.mods.unwrap().push(barony_mod)
                } else {
                    self.mods = Some(vec![barony_mod.clone()])
                }

                Command::perform(
                    download_image(
                        self.http_client.clone(),
                        barony_mod.workshop.preview_url.clone(),
                    ),
                    move |result| match result {
                        Ok(bytes) => {
                            Message::ModImageFetched(barony_mod.workshop.id.clone(), bytes)
                        }
                        Err(_msg) => Message::NoOp,
                    },
                )
            }
            Message::ModImageFetched(id, image_binary) => {
                dbg!(id);
                // if let Some(mods) = self.mods {
                //     mods.push(barony_mod) // self.mods.unwrap().push(barony_mod)
                // }
                Command::none()
            }
            // Message::ModReady(barony_mod) => Command::none(),
            Message::ErrorHappened(msg) => {
                self.error_message = Some(format!("An error occurred: {}", msg));
                Command::none()
            }
            Message::NoOp => Command::none(),
            Message::Scrolled(n) => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        // ------------------ Header -----------------------
        let app_name = Text::new("Barony Mod Manager")
            .size(30)
            .width(Length::FillPortion(3))
            .color(iced::Color::WHITE);

        let mod_search_input = TextInput::new(
            &mut self.mod_search_input,
            "Search for mods...",
            &self.query,
            Message::ModSearchInputChanged,
        )
        .padding(5)
        .width(Length::FillPortion(2))
        .style(BaronyModManagerUiStyles)
        .size(20);

        let header = Row::new().push(app_name).push(mod_search_input);

        // ------------------ Bottom inputs -----------------------
        let toggle_api_key_input_hidden = Checkbox::new(
            self.api_key_input_hidden,
            "Hide Steam API key",
            Message::ToggleHiddenApiKeyInput,
        )
        .size(20)
        .style(BaronyModManagerUiStyles)
        .text_size(20);

        let mut api_key_input = TextInput::new(
            &mut self.api_key_input,
            "Steam API Key",
            &self.steam_api_key,
            Message::ApiKeyInputChanged,
        )
        .padding(5)
        .style(BaronyModManagerUiStyles)
        .size(20);

        api_key_input = if self.api_key_input_hidden {
            api_key_input.password()
        } else {
            api_key_input
        };

        let api_key_section = Column::new()
            .spacing(15)
            .width(Length::FillPortion(2))
            .push(toggle_api_key_input_hidden)
            .push(api_key_input);

        let barony_path_input = TextInput::new(
            &mut self.barony_dir_input,
            "Barony directory",
            &self.barony_dir_str,
            Message::BaronyDirectoryPathChanged,
        )
        .padding(5)
        .style(BaronyModManagerUiStyles)
        .width(Length::FillPortion(2))
        .size(20);

        let bottom_inputs = Row::new()
            .push(api_key_section)
            .push(barony_path_input)
            .align_items(Align::End)
            .spacing(100);

        // ---------------- Filtering/Sorting ------------------
        let show_only_installed = Checkbox::new(
            self.show_only_installed,
            "List only installed mods",
            Message::ToggleShowOnlyInstalled,
        )
        .size(20)
        .style(BaronyModManagerUiStyles)
        .text_size(20);

        let pick_list_label = Text::new("Sort by").color(Color::WHITE);
        let pick_list = PickList::new(
            &mut self.sorter_picklist,
            &Sorter::ALL[..],
            self.selected_sorter.clone(),
            Message::SorterSelected,
        )
        .text_size(20)
        .style(BaronyModManagerUiStyles);

        let pick_list_tags = PickList::new(
            &mut self.tag_picklist,
            vec![
                PickableTag::Some(SteamWorkshopTag {
                    tag: "fooo".to_string(),
                    display_name: "fooo".to_string(),
                }),
                PickableTag::Some(SteamWorkshopTag {
                    tag: "barr".to_string(),
                    display_name: "barr".to_string(),
                }),
                PickableTag::None,
            ],
            self.selected_tag.clone(),
            Message::TagSelected,
        )
        .text_size(20)
        .style(BaronyModManagerUiStyles);

        let tag_pick_list_label = Text::new("Tag").color(Color::WHITE);
        let tag_pick_list = Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(tag_pick_list_label)
            .push(pick_list_tags);

        let pick_list_full = Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(pick_list_label)
            .push(pick_list);

        let search_options = Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(pick_list_full)
            .push(show_only_installed)
            .push(tag_pick_list);

        // ---------------- Mods container ------------------
        let button = Button::new(&mut self.button_state, Text::new("yeah"))
            .style(BaronyModManagerUiStyles)
            .on_press(Message::ButtonWasPressed);

        let main_section = if self.steam_api_key.is_empty() {
            let text =
                Text::new("Add your steam API key to the bottom left input and press enter.")
                    .color(Color::WHITE)
                    .size(35);
            Column::new().height(Length::Fill).push(text)
        } else if let Some(error) = &self.error_message {
            let text = Text::new(error).size(35).color(Color::WHITE);
            Column::new().height(Length::Fill).push(text)
        } else {
            let mut mods_scrollable = Scrollable::new(&mut self.mods_scrollable)
                .width(Length::Fill)
                .height(Length::Fill);
            // .on_scroll(Message::Scrolled);

            mods_scrollable = if let Some(mods) = self.mods.as_ref() {
                mods.into_iter().fold(mods_scrollable, |scroll, mod_| {
                    if let Some(image) = mod_.image_binary.clone() {
                        let handle = image::Handle::from_memory(image);
                        let image = Image::new(handle);
                        scroll.push(image)
                    } else {
                        scroll
                    }
                })
            } else {
                mods_scrollable
            };

            Column::new()
                .height(Length::Fill)
                .push(button)
                .push(mods_scrollable)
        };

        // -------------- Everything together --------------
        let all_content = Column::new()
            .spacing(20)
            .push(header)
            .push(search_options)
            .push(main_section)
            .push(bottom_inputs);

        Container::new(all_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(BaronyModManagerUiStyles)
            .into()
    }
}
