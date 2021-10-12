// Don't show the console when starting the app on Windows
#![windows_subsystem = "windows"]

use std::{collections::HashSet, time::Duration, vec};

use barony_mod_manager::{
    data::{BaronyMod, DownloadStatus},
    downloader_api::{check_status, download_mod, queue_download},
    filesystem::{self, barony_dir_valid},
    images::build_app_logo,
    steam_api::{build_barony_mod, get_barony_workshop_mods},
    styling::{
        DownloadModButton, DownloadingModButton, GeneralUiStyles, ModCardUiStyles, RemoveModButton,
    },
    widgets::{Filter, Message, PickableTag, Sorter, SortingStrategy},
};
use chrono::Datelike;
use iced::{
    button, executor, pick_list, scrollable, text_input, window, Align, Application, Button,
    Clipboard, Color, Column, Command, Container, Element, Image, Length, PickList, Row,
    Scrollable, Settings, Subscription, Text, TextInput,
};
use iced_native::Event;
use reqwest::Client;

static VERSION: &str = "v0.3.2";

fn main() -> iced::Result {
    let icon = build_app_logo().unwrap();

    BaronyModManager::run(iced::Settings {
        exit_on_close_request: false,
        window: window::Settings {
            icon: Some(icon),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

/// App state
struct BaronyModManager {
    // Core data
    mods: Option<Vec<BaronyMod>>,
    http_client: Client,

    // Mod querying
    mod_search_input: text_input::State,
    query: String,
    tags: HashSet<PickableTag>,

    // Sorters picklist
    sorter_picklist: pick_list::State<Sorter>,
    selected_sorter: Option<Sorter>,

    tag_picklist: pick_list::State<PickableTag>,
    selected_tag: Option<PickableTag>,

    sorting_strategy_picklist: pick_list::State<SortingStrategy>,
    sorting_strategy: Option<SortingStrategy>,

    loading_mods: bool,

    // Barony dir input
    barony_dir_str: String,
    barony_dir_input: text_input::State,
    barony_dir_valid: bool,

    // Button
    search_button_state: button::State,
    filter_picklist: pick_list::State<Filter>,
    selected_filter: Option<Filter>,

    mods_scrollable: scrollable::State,

    // Misc
    should_exit: bool,
    error_message: Option<String>,
}

impl Application for BaronyModManager {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn title(&self) -> String {
        format!("Barony Mod Manager {}", VERSION)
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
        let barony_dir = persisted_settings.barony_directory_path.unwrap_or_default();

        let initial_state = BaronyModManager {
            mods: None,

            http_client: Client::new(),
            tags: HashSet::new(),
            // Mod querying
            mod_search_input: text_input::State::default(),
            query: "".to_string(),
            // Pick list
            sorter_picklist: pick_list::State::default(),
            selected_sorter: Some(Sorter::default()),

            sorting_strategy_picklist: pick_list::State::default(),
            sorting_strategy: Some(SortingStrategy::default()),

            loading_mods: false,
            tag_picklist: pick_list::State::default(),
            selected_tag: Some(PickableTag::default()),

            barony_dir_valid: barony_dir_valid(&barony_dir),
            barony_dir_str: barony_dir,
            barony_dir_input: text_input::State::default(),

            search_button_state: button::State::default(),

            mods_scrollable: scrollable::State::default(),

            filter_picklist: pick_list::State::default(),
            selected_filter: Some(Filter::default()),

            should_exit: false,
            error_message: None,
        };

        let duration = Duration::from_millis(1);
        (
            initial_state,
            Command::perform(async_std::task::sleep(duration), |_| Message::LoadMods),
        )
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::ModSearchInputChanged(new_value) => {
                self.query = new_value;
                Command::none()
            }
            Message::LoadMods => {
                self.mods = None;
                self.loading_mods = true;
                Command::perform(
                    get_barony_workshop_mods(self.http_client.clone()),
                    |result| match result {
                        Ok(workshop_mods) => Message::ModsFetched(workshop_mods),
                        Err(message) => Message::ErrorHappened(message.to_string()),
                    },
                )
            }
            Message::SortingStrategySelected(new_strategy) => {
                self.sorting_strategy = Some(new_strategy);
                sort_mods(self);
                Command::none()
            }
            Message::SorterSelected(new_sorter) => {
                self.selected_sorter = Some(new_sorter);
                sort_mods(self);
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
                self.barony_dir_valid = barony_dir_valid(&new_value);
                self.barony_dir_str = new_value;
                Command::none()
            }
            Message::CloseRequested => {
                // Cleanup
                filesystem::persist_settings(filesystem::SettingsPersistance {
                    barony_directory_path: Some(self.barony_dir_str.clone()),
                });
                self.should_exit = true;
                Command::none()
            }
            Message::ModsFetched(steam_workshop_mods) => {
                for mod_ in &steam_workshop_mods {
                    for tag in &mod_.tags {
                        let pickable = PickableTag::Some(tag.clone());
                        self.tags.insert(pickable);
                    }
                }

                Command::batch(steam_workshop_mods.into_iter().map(|mod_| {
                    Command::perform(
                        build_barony_mod(
                            self.http_client.clone(),
                            self.barony_dir_str.clone(),
                            mod_,
                        ),
                        Message::ModBuilt,
                    )
                }))
            }
            Message::ModBuilt(barony_mod) => {
                self.loading_mods = false;
                self.error_message = None;

                if let Some(mods) = &mut self.mods {
                    mods.push(barony_mod) // self.mods.unwrap().push(barony_mod)
                } else {
                    self.mods = Some(vec![barony_mod])
                }

                Command::none()
            }
            Message::DownloadMod(id) => {
                // TODO: Make function for this
                let selected_mod = self
                    .mods
                    .as_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|_mod| _mod.workshop.id == id)
                    .unwrap();

                selected_mod.download_status = DownloadStatus::Preparing;
                Command::perform(
                    queue_download(self.http_client.clone(), id.parse::<u32>().unwrap()),
                    move |result| match result {
                        Ok(uuid) => Message::PreparingModDownload(id.clone(), uuid),
                        _ => todo!(),
                    },
                )
            }
            Message::PreparingModDownload(id, uuid) => Command::perform(
                check_status(self.http_client.clone(), uuid.clone()),
                move |result| match result {
                    Ok(true) => Message::ModDownloadReady(id.clone(), uuid.clone()),
                    Ok(false) => Message::PreparingModDownload(id.clone(), uuid.clone()),
                    Err(err) => {
                        dbg!(err);
                        todo!()
                    }
                },
            ),
            Message::ModDownloadReady(id, uuid) => {
                let selected_mod = self
                    .mods
                    .as_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|_mod| _mod.workshop.id == id)
                    .unwrap();

                selected_mod.download_status = DownloadStatus::Downloading;

                // TODO: Maybe try to use less clones here, but since it's a function
                // that will run asynchronously, I'm not sure if it's possible without
                // huge pain
                let barony_dir = self.barony_dir_str.clone();
                let mod_title = selected_mod.workshop.title.clone();

                Command::perform(
                    download_mod(self.http_client.clone(), uuid),
                    move |result| match result {
                        Ok(zip_bytes) => {
                            filesystem::write_mod_to_disk(
                                barony_dir.clone(),
                                mod_title.clone(),
                                zip_bytes,
                            )
                            .unwrap();
                            Message::ModDownloaded(id.clone())
                        }
                        _ => todo!(),
                    },
                )
            }
            Message::ModDownloaded(id) => {
                let selected_mod = self
                    .mods
                    .as_mut()
                    .unwrap()
                    .iter_mut()
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
                    .iter_mut()
                    .find(|_mod| _mod.workshop.id == id)
                    .unwrap();

                // TODO: treat error
                filesystem::delete_mod_from_disk(
                    &self.barony_dir_str,
                    &selected_mod.workshop.title,
                )
                .unwrap();

                selected_mod.download_status = DownloadStatus::NotDownloaded;
                Command::none()
            }
            Message::ErrorHappened(msg) => {
                self.error_message = Some(format!("An error occurred: {}", msg));
                Command::none()
            }
        }
    }

    // TODO: Clean this view
    fn view(&mut self) -> Element<Self::Message> {
        // ------------------ Header -----------------------
        let app_name = Text::new(format!("Barony Mod Manager {}", VERSION))
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

        let path_label_message = format!(
            "Barony directory {}",
            if self.barony_dir_valid {
                "(VALID)"
            } else {
                "(INVALID)"
            }
        );

        let barony_path_label = Text::new(path_label_message).size(20).color(Color::WHITE);
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
            .max_width(600)
            .push(barony_path_label)
            .push(barony_path_input);

        let bottom_inputs = Row::new()
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

        let sorting_strategy_label = Text::new("Order:").color(Color::WHITE);
        let sorting_strategy_picklist = PickList::new(
            &mut self.sorting_strategy_picklist,
            &SortingStrategy::ALL[..],
            self.sorting_strategy.clone(),
            Message::SortingStrategySelected,
        )
        .text_size(20)
        .style(GeneralUiStyles);

        let sorting_strategy = Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(sorting_strategy_label)
            .push(sorting_strategy_picklist);

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
            .push(tag_pick_list)
            .push(sorting_strategy);

        let search_options = Row::new().push(search_options_).push(refresh_section);

        // ---------------- Mods container ------------------
        let main_section = if let Some(error) = &self.error_message {
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
            let mods_scrollable = if let Some(mods) = &mut self.mods {
                let mods_scrollable = Scrollable::new(&mut self.mods_scrollable)
                    .padding(15)
                    .spacing(15)
                    .width(Length::Fill)
                    .height(Length::Fill);

                // TODO: Don't filter mods every render
                let download_filtered = if let Some(filter) = &self.selected_filter {
                    mods.iter_mut()
                        .filter(|mod_| match filter {
                            Filter::Downloaded => {
                                mod_.download_status == DownloadStatus::Downloaded
                            }
                            Filter::NonDownloaded => {
                                mod_.download_status == DownloadStatus::NotDownloaded
                            }
                            Filter::Downloading => {
                                mod_.download_status == DownloadStatus::Downloading
                                    || mod_.download_status == DownloadStatus::Preparing
                            }
                            Filter::None => true,
                        })
                        .collect::<Vec<_>>()
                } else {
                    mods.iter_mut().collect::<Vec<_>>()
                };

                let tags_filtered = if let Some(tag) = &self.selected_tag {
                    match tag {
                        PickableTag::Some(tag) => download_filtered
                            .into_iter()
                            .filter(|mod_| {
                                mod_.workshop
                                    .clone()
                                    .tags
                                    .into_iter()
                                    .any(|mod_tag| mod_tag == *tag)
                            })
                            .collect::<Vec<_>>(),
                        PickableTag::None => download_filtered,
                    }
                } else {
                    download_filtered
                };

                // Filter by user search
                let mods_filtered = if !self.query.is_empty() {
                    let query = self.query.to_lowercase();
                    tags_filtered
                        .into_iter()
                        .filter(|mod_| {
                            mod_.workshop.title.to_lowercase().contains(&query)
                                || mod_.workshop.description.to_lowercase().contains(&query)
                        })
                        .collect::<Vec<_>>()
                } else {
                    tags_filtered
                };

                mods_filtered
                    .into_iter()
                    .fold(mods_scrollable, |scroll, mod_| {
                        let mod_image = Image::new(mod_.image_handle.clone());

                        let views_label = Text::new(format!("Views: {}", mod_.workshop.views))
                            .color(Color::WHITE);

                        let votes_up_label = Text::new(format!("Up: {}", mod_.workshop.votes.up))
                            .color(Color::WHITE);

                        let votes_down_label =
                            Text::new(format!("Down: {}", mod_.workshop.votes.down))
                                .color(Color::WHITE);

                        let votes_row = Row::new()
                            .spacing(5)
                            .push(votes_up_label)
                            .push(votes_down_label);

                        // TODO: Create function for this
                        let created_at = Text::new(format!(
                            "Created: {}/{}/{}",
                            mod_.workshop.time_created.day(),
                            mod_.workshop.time_created.month(),
                            mod_.workshop.time_created.year()
                        ))
                        .color(Color::WHITE);

                        let last_updated_at = Text::new(format!(
                            "Updated: {}/{}/{}",
                            mod_.workshop.time_updated.day(),
                            mod_.workshop.time_updated.month(),
                            mod_.workshop.time_updated.year()
                        ))
                        .color(Color::WHITE);

                        let download_or_remove_button = match mod_.download_status {
                            // An error occured or the mod is not downloaded, can try again
                            DownloadStatus::NotDownloaded | DownloadStatus::ErrorOccurred => {
                                Button::new(&mut mod_.download_button, Text::new("Download"))
                                    .style(DownloadModButton)
                                    .on_press(Message::DownloadMod(mod_.workshop.id.clone()))
                            }
                            DownloadStatus::Downloading | DownloadStatus::Preparing => {
                                Button::new(&mut mod_.download_button, Text::new("Downloading"))
                                    .style(DownloadingModButton)
                            }
                            _ => Button::new(&mut mod_.download_button, Text::new("Remove"))
                                .style(RemoveModButton)
                                .on_press(Message::RemoveMod(mod_.workshop.id.clone())),
                        };

                        let buttons_row = Column::new().spacing(10).push(download_or_remove_button);

                        // TODO: Don't unwrap this here (if it crashes will explode the program)
                        let bytes_size = mod_.workshop.file_size;
                        let size_text = Text::new(format!(
                            "Size: {:.2}MB",
                            (bytes_size as f64) / 1024.0 / 1024.0
                        ))
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
                Scrollable::new(&mut self.mods_scrollable)
                    .padding(15)
                    .spacing(15)
                    .width(Length::Fill)
                    .height(Length::Fill)
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

fn sort_mods(state: &mut BaronyModManager) {
    if let Some(sorter) = &state.selected_sorter {
        if let Some(mods) = &mut state.mods {
            match sorter {
                Sorter::None => (),
                other => mods.sort_unstable_by(|a, b| match other {
                    Sorter::Size => a.workshop.file_size.cmp(&b.workshop.file_size),
                    Sorter::Views => a.workshop.views.cmp(&b.workshop.views),
                    Sorter::Created => a.workshop.time_created.cmp(&b.workshop.time_created),
                    Sorter::Updated => a.workshop.time_updated.cmp(&b.workshop.time_updated),
                    Sorter::VoteScore => a.workshop.votes.up.cmp(&b.workshop.votes.up),
                    Sorter::None => panic!("Should never match"),
                }),
            };

            if let Some(SortingStrategy::Descending) = state.sorting_strategy {
                mods.reverse();
            }
        }
    }
}
