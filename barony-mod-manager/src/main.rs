use std::path::PathBuf;

use barony_mod_manager::{
    data::BaronyMod,
    filesystem,
    steam_api::steam_request,
    styling::BaronyModManagerUiStyles,
    widgets::{Message, Sorter},
};
use iced::{
    button, executor, futures::io::Window, pick_list, text_input, window, Align, Application,
    Button, Checkbox, Clipboard, Color, Column, Command, Container, Element, Length, PickList, Row,
    Settings, Subscription, Text, TextInput,
};
use iced_native::Event;

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

    // Show only installed checkbox
    show_only_installed: bool,

    // Barony dir input
    barony_dir_str: String,
    barony_dir_input: text_input::State,

    // Button
    button_state: button::State,

    // Misc
    should_exit: bool,
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

            barony_dir_str: persisted_settings.barony_directory_path.unwrap_or_default(),
            barony_dir_input: text_input::State::default(),

            button_state: button::State::default(),

            should_exit: false,
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
            Message::ButtonWasPressed => {
                Command::perform(steam_request("flsadf".to_string()), |a| {
                    if a == "success".to_string() {
                        Message::SorterSelected(Sorter::VoteScore)
                    } else {
                        Message::SorterSelected(Sorter::Subscribed)
                    }
                })
            }
            Message::SorterSelected(new_sorter) => {
                self.selected_sorter = Some(new_sorter);
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
            "Hide API Key Input",
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

        let pick_list_full = Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(pick_list_label)
            .push(pick_list);

        let search_options = Row::new()
            .push(pick_list_full)
            .push(show_only_installed)
            .align_items(Align::Center)
            .spacing(20);

        // ---------------- Mods container ------------------
        let button = Button::new(&mut self.button_state, Text::new("yeah"))
            .on_press(Message::ButtonWasPressed);
        let mods = Column::new().height(Length::Fill).push(button);

        // -------------- Everything together --------------
        let all_content = Column::new()
            .spacing(20)
            .push(header)
            .push(search_options)
            .push(mods)
            .push(bottom_inputs);

        Container::new(all_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(BaronyModManagerUiStyles)
            .into()
    }
}
