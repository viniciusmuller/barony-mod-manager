use std::{collections::HashSet, time::Duration, vec};

use barony_mod_manager::{
    data::{BaronyMod, DownloadStatus},
    filesystem,
    steam_api::{get_total_mods, get_workshop_item},
    styling::{GeneralUiStyles, ModCardUiStyles},
    widgets::{Filter, Message, PickableTag, Sorter},
};
use chrono::Datelike;
use iced::{
    button, executor, pick_list, scrollable, text_input, Align, Application, Button, Checkbox,
    Clipboard, Color, Column, Command, Container, Element, Image, Length, PickList, Row,
    Scrollable, Settings, Subscription, Text, TextInput,
};
use iced_native::Event;
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
    // barony_dir: Option<PathBuf>,
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
    loading_mods: bool,

    // Barony dir input
    barony_dir_str: String,
    barony_dir_input: text_input::State,

    // Button
    search_button_state: button::State,
    filter_picklist: pick_list::State<Filter>,
    selected_filter: Option<Filter>,

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
        iced_native::subscription::events_with(|event, _other| match event {
            // Listen only for window close requests, don't triggering unnecessary renders
            Event::Window(iced_native::window::Event::CloseRequested) => {
                Some(Message::CloseRequested)
            }
            _ => None,
        })
    }

    fn new(_flags: Self::Flags) -> (BaronyModManager, Command<Message>) {
        let persisted_settings = filesystem::load_persisted_settings();

        let initial_state = BaronyModManager {
            // barony_dir: None,
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

            loading_mods: false,
            tag_picklist: pick_list::State::default(),
            selected_tag: Some(PickableTag::default()),

            barony_dir_str: persisted_settings.barony_directory_path.unwrap_or_default(),
            barony_dir_input: text_input::State::default(),

            // TODO: Remove
            search_button_state: button::State::default(),

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
            Message::LoadMods => {
                self.mods = None;
                self.loading_mods = true;
                Command::perform(
                    get_total_mods(self.http_client.clone(), self.steam_api_key.clone()),
                    |result| match result {
                        Ok(number) => Message::TotalModsNumber(number),
                        Err(message) => Message::ErrorHappened(message),
                    },
                )
            }
            Message::TotalModsNumber(total) => iced::Command::batch((1..=total).map(|n| {
                Command::perform(
                    get_workshop_item(self.http_client.clone(), self.steam_api_key.clone(), n),
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
            Message::CloseRequested => {
                // Cleanup
                filesystem::persist_settings(filesystem::SettingsPersistance {
                    barony_directory_path: Some(self.barony_dir_str.clone()),
                    steam_api_key: Some(self.steam_api_key.clone()),
                });
                self.should_exit = true;
                Command::none()
            }
            Message::ModFetched(barony_mod) => {
                self.loading_mods = false;
                self.error_message = None;

                if let Some(mods) = &mut self.mods {
                    mods.push(barony_mod.clone()) // self.mods.unwrap().push(barony_mod)
                } else {
                    self.mods = Some(vec![barony_mod.clone()])
                }

                for tag in &barony_mod.workshop.tags {
                    let pickable = PickableTag::Some(tag.clone());
                    self.tags.insert(pickable);
                }

                Command::none()
            }
            Message::ToggleActivateMod(id, flag) => {
                let selected_mod = self
                    .mods
                    .as_mut()
                    .unwrap()
                    .into_iter()
                    .find(|_mod| _mod.workshop.id == id)
                    .unwrap();

                // If the mod is downloaded, it can be activated/deactivated
                // let mod_can_change = match selected_mod.download_status {
                //     DownloadStatus::Downloaded => true,
                //     _ => false
                // };

                match selected_mod.download_status {
                    DownloadStatus::Downloaded => {
                        selected_mod.is_active = flag;
                    }
                    _ => (),
                }

                Command::none()
            }
            Message::DownloadMod(id) => {
                // TODO: Make function for this
                let selected_mod = self
                    .mods
                    .as_mut()
                    .unwrap()
                    .into_iter()
                    .find(|_mod| _mod.workshop.id == id)
                    .unwrap();

                selected_mod.is_downloaded = true;
                let duration = Duration::from_millis(1);
                Command::perform(async_std::task::sleep(duration), move |_| {
                    Message::PreparingModDownload(id.clone())
                })
            }
            Message::PreparingModDownload(id) => {
                let selected_mod = self
                    .mods
                    .as_mut()
                    .unwrap()
                    .into_iter()
                    .find(|_mod| _mod.workshop.id == id)
                    .unwrap();

                selected_mod.download_status = DownloadStatus::Preparing;
                let duration = Duration::from_millis(800);
                Command::perform(async_std::task::sleep(duration), move |_| {
                    Message::ModDownloadReady(id.clone())
                })
            }
            Message::ModDownloadReady(id) => {
                let selected_mod = self
                    .mods
                    .as_mut()
                    .unwrap()
                    .into_iter()
                    .find(|_mod| _mod.workshop.id == id)
                    .unwrap();

                selected_mod.download_status = DownloadStatus::Downloading;
                let duration = Duration::from_millis(1500);
                Command::perform(async_std::task::sleep(duration), move |_| {
                    Message::ModDownloaded(id.clone())
                })
            }
            Message::ModDownloaded(id) => {
                let selected_mod = self
                    .mods
                    .as_mut()
                    .unwrap()
                    .into_iter()
                    .find(|_mod| _mod.workshop.id == id)
                    .unwrap();

                selected_mod.download_status = DownloadStatus::Downloaded;
                Command::none()
            }
            Message::RemoveMod(id) => {
                let selected_mod = self
                    .mods
                    .as_mut()
                    .unwrap()
                    .into_iter()
                    .find(|_mod| _mod.workshop.id == id)
                    .unwrap();

                // TODO: Remove mod files here

                selected_mod.is_active = false;
                selected_mod.download_status = DownloadStatus::NotDownloaded;
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

        let barony_path_label = Text::new("Barony directory").size(20).color(Color::WHITE);
        let barony_path_input = TextInput::new(
            &mut self.barony_dir_input,
            "Barony directory",
            &self.barony_dir_str,
            Message::BaronyDirectoryPathChanged,
        )
        .padding(5)
        .style(GeneralUiStyles)
        .size(20);

        let barony_path_section = Column::new()
            .spacing(10)
            .width(Length::FillPortion(2))
            .push(barony_path_label)
            .push(barony_path_input);

        let bottom_inputs = Row::new()
            .push(api_key_section)
            .push(barony_path_section)
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

        let refresh_button = Button::new(&mut self.search_button_state, Text::new("Refresh"))
            .style(GeneralUiStyles)
            .width(Length::Shrink)
            .on_press(Message::LoadMods);

        let refresh_section = Row::new().width(Length::Shrink).push(refresh_button);

        let search_options_ = Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(pick_list_full)
            .push(filter_pick_list_)
            .push(tag_pick_list);

        let search_options = Row::new().push(search_options_).push(refresh_section);

        // ---------------- Mods container ------------------
        let main_section = if self.steam_api_key.is_empty() {
            let text = Text::new(
                "Add your steam API key to the bottom left input and click the \"Refresh\" button.",
            )
            .color(Color::WHITE)
            .size(35);

            // TODO: Create function for those repeated aligned container creations
            Container::new(text)
                .align_x(Align::Center)
                .align_y(Align::Center)
                .width(Length::Fill)
                .height(Length::Fill)
        } else if let Some(error) = &self.error_message {
            let text = Text::new(error).size(35).color(Color::WHITE);

            Container::new(text)
                .height(Length::Fill)
                .width(Length::Fill)
                .align_x(Align::Center)
                .align_y(Align::Center)
        } else if self.loading_mods {
            let text = Text::new("Loading mods...").color(Color::WHITE).size(35);

            Container::new(text)
                .align_x(Align::Center)
                .align_y(Align::Center)
                .width(Length::Fill)
                .height(Length::Fill)
        } else if self.mods.is_none() {
            let text = Text::new(
                "You have not loaded the available mods yet. Click the \"Refresh\" button to load them.",
            )
            .size(30)
            .color(Color::WHITE);

            Container::new(text)
                .height(Length::Fill)
                .width(Length::Fill)
                .align_x(Align::Center)
                .align_y(Align::Center)
        } else {
            let mut mods_scrollable = Scrollable::new(&mut self.mods_scrollable)
                .padding(15)
                .spacing(15)
                .width(Length::Fill)
                .height(Length::Fill);

            // TODO: Can be safely unwrapped here.
            mods_scrollable = if let Some(mods) = &mut self.mods {
                // TODO: Filter mods here

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

                    let id = mod_.workshop.id.clone();
                    let activate_checkbox =
                        Checkbox::new(mod_.is_active, "Activated", move |value| {
                            Message::ToggleActivateMod(id.clone(), value)
                        })
                        .spacing(8)
                        .style(GeneralUiStyles);

                    let download_or_remove_button = match mod_.download_status {
                        DownloadStatus::Downloaded => {
                            Button::new(&mut mod_.download_button, Text::new("Remove"))
                                .style(GeneralUiStyles)
                                .on_press(Message::RemoveMod(mod_.workshop.id.clone()))
                        }
                        DownloadStatus::NotDownloaded | DownloadStatus::ErrorOccurred => {
                            Button::new(&mut mod_.download_button, Text::new("Download"))
                                .style(GeneralUiStyles)
                                .on_press(Message::DownloadMod(mod_.workshop.id.clone()))
                        }
                        // Remove on_press since it's already downloading
                        _ => Button::new(&mut mod_.download_button, Text::new("Downloading"))
                            .style(GeneralUiStyles),
                    };

                    let buttons_row = Column::new()
                        .spacing(10)
                        .push(download_or_remove_button)
                        .push(activate_checkbox);

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
                        .push(size_col)
                        .push(buttons_row);

                    let mod_title = Text::new(mod_.workshop.title.clone())
                        .size(25)
                        .color(Color::WHITE);

                    let description = mod_
                        .workshop
                        .description
                        .chars()
                        .into_iter()
                        .take(2000)
                        .collect::<String>();

                    let description = if description.len() >= 2000 {
                        format!("{}...", description)
                    } else {
                        description
                    };

                    let mod_description = Text::new(description).color(Color::WHITE);

                    let mod_info_description = Column::new()
                        .spacing(10)
                        .push(mod_title)
                        .push(mod_description);

                    let status_message = format!("Status: {}", mod_.download_status);
                    let mod_download_status =
                        Column::new().push(Text::new(status_message).color(Color::WHITE));

                    let mod_info = Column::new()
                        .spacing(10)
                        .push(mod_info_description)
                        .push(mod_download_status);

                    let mod_card = Row::new().spacing(20).push(image_col).push(mod_info);

                    let container = Container::new(mod_card)
                        .padding(10)
                        .width(Length::Fill)
                        .style(ModCardUiStyles);

                    scroll.push(container)
                })
            } else {
                mods_scrollable
            };

            Container::new(mods_scrollable).height(Length::Fill)
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
