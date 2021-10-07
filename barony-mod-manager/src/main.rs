use std::{collections::HashSet, path::PathBuf, vec};

use barony_mod_manager::{
    data::BaronyMod,
    filesystem,
    steam_api::{get_total_mods, get_workshop_item},
    styling::GeneralUiStyles,
    widgets::{Filter, Message, PickableTag, Sorter},
};
use chrono::Datelike;
use iced::{
    button, executor, pick_list, scrollable, text_input, Align, Application, Button,
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
    tags: HashSet<PickableTag>,

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
    search_button_state: button::State,
    filter_picklist: pick_list::State<Filter>,
    selected_filter: Option<Filter>,

    download_buttons: Vec<button::State>,
    activate_buttons: Vec<button::State>,

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
            tags: HashSet::new(),
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

            // TODO: Remove
            search_button_state: button::State::default(),

            download_buttons: vec![],
            activate_buttons: vec![],

            mods_scrollable: scrollable::State::default(),

            filter_picklist: pick_list::State::default(),
            selected_filter: Some(Filter::default()),

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
            Message::FilterSelected(filter) => {
                self.selected_filter = Some(filter);
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

                for tag in &barony_mod.workshop.tags {
                    let pickable = PickableTag::Some(tag.clone());
                    self.tags.insert(pickable);
                }

                // Command::perform(
                //     download_image(
                //         self.http_client.clone(),
                //         barony_mod.workshop.preview_url.clone(),
                //     ),
                //     move |result| match result {
                //         Ok(image) => {
                //             Message::ModImageFetched(barony_mod.workshop.id.clone(), image)
                //         }
                //         Err(_msg) => Message::NoOp,
                //     },
                // )

                Command::none()
            }
            // Message::ModImageFetched(id, image) => {
            //     if let Some(mods) = &mut self.mods {
            //         let index = mods.into_iter().position(|m| m.workshop.id == id).unwrap();
            //         let barony_mod = mods.get_mut(index).unwrap();
            //         barony_mod.image_handle = Some(image)
            //     }

            //     Command::none()
            // }
            Message::TestButtonPressed => {
                dbg!("Button was pressed");
                Command::none()
            }
            Message::ErrorHappened(msg) => {
                self.error_message = Some(format!("An error occurred: {}", msg));
                Command::none()
            }
            Message::NoOp => Command::none(),
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
        .style(GeneralUiStyles)
        .size(20);

        let header = Row::new().push(app_name).push(mod_search_input);

        // ------------------ Bottom inputs -----------------------
        let toggle_api_key_input_hidden = Checkbox::new(
            self.api_key_input_hidden,
            "Hide Steam API key",
            Message::ToggleHiddenApiKeyInput,
        )
        .size(20)
        .style(GeneralUiStyles)
        .text_size(20);

        let mut api_key_input = TextInput::new(
            &mut self.api_key_input,
            "Steam API Key",
            &self.steam_api_key,
            Message::ApiKeyInputChanged,
        )
        .padding(5)
        .style(GeneralUiStyles)
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
        .style(GeneralUiStyles)
        .width(Length::FillPortion(2))
        .size(20);

        let bottom_inputs = Row::new()
            .push(api_key_section)
            .push(barony_path_input)
            .align_items(Align::End)
            .spacing(100);

        // ---------------- Filtering/Sorting ------------------
        let filter_picklist_label = Text::new("Filter:").color(Color::WHITE);
        let filter_pick_list = PickList::new(
            &mut self.filter_picklist,
            &Filter::ALL[..],
            self.selected_filter.clone(),
            Message::FilterSelected,
        )
        .style(GeneralUiStyles);

        let filter_pick_list_ = Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(filter_picklist_label)
            .push(filter_pick_list);

        let pick_list_label = Text::new("Sort by:").color(Color::WHITE);
        let pick_list = PickList::new(
            &mut self.sorter_picklist,
            &Sorter::ALL[..],
            self.selected_sorter.clone(),
            Message::SorterSelected,
        )
        .text_size(20)
        .style(GeneralUiStyles);

        let mut tags = self.tags.clone().into_iter().collect::<Vec<_>>();
        tags.push(PickableTag::None);
        tags.sort();

        let pick_list_tags = PickList::new(
            &mut self.tag_picklist,
            tags,
            self.selected_tag.clone(),
            Message::TagSelected,
        )
        .text_size(20)
        .style(GeneralUiStyles);

        let tag_pick_list_label = Text::new("Tag:").color(Color::WHITE);
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
            .push(filter_pick_list_)
            .push(tag_pick_list);

        // ---------------- Mods container ------------------
        let button = Button::new(&mut self.search_button_state, Text::new("Search"))
            .style(GeneralUiStyles)
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
                .spacing(20)
                .width(Length::Fill)
                .height(Length::Fill);

            mods_scrollable = if let Some(mods) = &self.mods {
                // TODO: Filter mods here

                // let download_buttons = &mut self.download_buttons;
                // let no_image = self.mod_no_image_image;

                mods.into_iter().fold(mods_scrollable, |scroll, mod_| {
                    let mod_image = Image::new(mod_.image_handle.clone());

                    let views_label =
                        Text::new(format!("Views: {}", mod_.workshop.views)).color(Color::WHITE);

                    let votes_up_label =
                        Text::new(format!("Up: {}", mod_.workshop.vote_data.votes_up))
                            .color(Color::WHITE);

                    let votes_down_label =
                        Text::new(format!("Down: {}", mod_.workshop.vote_data.votes_down))
                            .color(Color::WHITE);

                    let votes_row = Row::new()
                        .spacing(5)
                        .push(votes_up_label)
                        .push(votes_down_label);

                    // TODO: Create function for this
                    let created_at = Text::new(format!(
                        "Created: {}/{}/{}",
                        mod_.workshop.time_updated.day(),
                        mod_.workshop.time_updated.month(),
                        mod_.workshop.time_updated.year()
                    ))
                    .color(Color::WHITE);

                    let last_updated_at = Text::new(format!(
                        "Updated: {}/{}/{}",
                        mod_.workshop.time_updated.day(),
                        mod_.workshop.time_updated.month(),
                        mod_.workshop.time_updated.year()
                    ))
                    .color(Color::WHITE);

                    // let mut download_button_state = button::State::default();
                    // download_buttons.push(download_button_state);
                    // let download_button =
                    //     Button::new(&mut download_button_state, Text::new("Download"))
                    //         .on_press(Message::TestButtonPressed);

                    // let activate_button = Button::new(
                    //     &mut self.dummy_activate_button_state,
                    //     Text::new("Activate"),
                    // )
                    // .on_press(Message::TestButtonPressed);

                    // let buttons_row = Row::new().push(download_button); //.push(activate_button);

                    // TODO: Don't unwrap this here (if it crashes will explode the program)
                    let bytes_size = mod_.workshop.file_size.parse::<f64>().unwrap();
                    let size_text =
                        Text::new(format!("Size: {:.2}MB", bytes_size / 1024.0 / 1024.0))
                            .color(Color::WHITE);

                    let size_col = Column::new().push(size_text);

                    let dates_col = Column::new()
                        .spacing(10)
                        .push(created_at)
                        .push(last_updated_at);

                    let image_col = Column::new()
                        .spacing(10)
                        .push(mod_image)
                        .push(views_label)
                        .push(votes_row)
                        .push(dates_col)
                        .push(size_col);
                    // .push(buttons_row);

                    let mod_title = Text::new(mod_.workshop.title.clone())
                        .size(25)
                        .color(Color::WHITE);

                    let mod_description =
                        Text::new(mod_.workshop.description.clone()).color(Color::WHITE);

                    let mod_info = Column::new()
                        .spacing(10)
                        .push(mod_title)
                        .push(mod_description);

                    // let mut btnstate = button::State::default();
                    // self.download_buttons.push(btnstate);
                    // let download_button = Button::new(&mut btnstate, Text::new("Download"))
                    //     .on_press(Message::ButtonWasPressed);

                    // let misc_mod_info = Column::new().push(download_button);

                    let mod_card = Row::new().spacing(20).push(image_col).push(mod_info);
                    // .push(misc_mod_info);

                    scroll.push(mod_card)
                    // } else {
                    //     scroll
                    // }
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
            .style(GeneralUiStyles)
            .into()
    }
}
