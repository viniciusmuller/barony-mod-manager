use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
};

use barony_mod_manager::{data::BaronyMod, styling::BaronyModManagerUiStyles};
use iced::{
    button, executor, pick_list, text_input, Align, Application, Button, Checkbox, Clipboard,
    Color, Column, Command, Container, Element, Length, PickList, Row, Settings, Text, TextInput,
};

fn main() -> iced::Result {
    BaronyModManager::run(Settings::default())
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
}

#[derive(Clone, Debug)]
enum Message {
    ApiKeyInputChanged(String),
    ModSearchInputChanged(String),
    ToggleHiddenApiKeyInput(bool),
    ToggleShowOnlyInstalled(bool),
    SorterSelected(Sorter),
    ButtonWasPressed,
}

/// Possible fields that can be used to sort the mods
#[derive(Clone, Debug)]
enum Sorter {
    VoteScore,
    Views,
    Subscribed,
    Updated,
    Created,
    None,
}

impl Sorter {
    const ALL: [Sorter; 6] = [
        Sorter::VoteScore,
        Sorter::Views,
        Sorter::Subscribed,
        Sorter::Updated,
        Sorter::Created,
        Sorter::None,
    ];
}

impl Display for Sorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sorter::VoteScore => "Vote score",
                Sorter::Views => "Views",
                Sorter::Subscribed => "Subscribed",
                Sorter::Updated => "Date updated",
                Sorter::Created => "Date created",
                Sorter::None => "Nothing",
            }
        )
    }
}

impl PartialEq for Sorter {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
impl Eq for Sorter {}

impl Default for Sorter {
    fn default() -> Sorter {
        Sorter::None
    }
}

// TODO: Create a workshop items `Tag` picklist

impl Application for BaronyModManager {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (BaronyModManager, Command<Message>) {
        let initial_state = BaronyModManager {
            barony_dir: None,
            mods: None,
            // Steam API key
            api_key_input: text_input::State::default(),
            api_key_input_hidden: true,
            steam_api_key: "".to_owned(),
            // Mod querying
            mod_search_input: text_input::State::default(),
            query: "".to_string(),
            // Pick list
            sorter_picklist: pick_list::State::default(),
            selected_sorter: Some(Sorter::default()),
            show_only_installed: false,
        };

        (initial_state, Command::none())
    }

    fn title(&self) -> String {
        String::from("Barony Mod Manager")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        // This application has no interactions
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
            Message::ButtonWasPressed => Command::none(),
            Message::SorterSelected(new_sorter) => {
                self.selected_sorter = Some(new_sorter);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let app_name = Text::new("Barony Mod Manager")
            .size(30)
            .width(Length::FillPortion(3))
            .color(iced::Color::WHITE);

        let mut api_key_input = TextInput::new(
            &mut self.api_key_input,
            "Steam API Key",
            &self.steam_api_key,
            Message::ApiKeyInputChanged,
        )
        .padding(5)
        .style(BaronyModManagerUiStyles)
        .size(20);

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

        api_key_input = if self.api_key_input_hidden {
            api_key_input.password()
        } else {
            api_key_input
        };

        let toggle_api_key_input_hidden = Checkbox::new(
            self.api_key_input_hidden,
            "Hide API Key Input",
            Message::ToggleHiddenApiKeyInput,
        )
        .size(20)
        .style(BaronyModManagerUiStyles)
        .text_size(20);

        let top = Row::new().push(app_name).push(mod_search_input);

        let api_key_section = Column::new()
            .spacing(15)
            .max_width(500)
            .push(toggle_api_key_input_hidden)
            .push(api_key_input);

        // let button = Button::new(&mut self.test_button, Text::new("Button"))
        //     .on_press(Message::ButtonWasPressed)
        //     .style(BaronyModManagerUiStyles);

        // Things such as `show only installed flag`, `sort by`
        //
        let show_only_installed = Checkbox::new(
            self.show_only_installed,
            "Show only installed mods",
            Message::ToggleShowOnlyInstalled,
        )
        .size(20)
        // .width(Length::FillPortion(1))
        .style(BaronyModManagerUiStyles)
        .text_size(20);

        let pick_list_label = Text::new("Sort by").color(Color::WHITE);
        let pick_list = pick_list::PickList::new(
            &mut self.sorter_picklist,
            &Sorter::ALL[..],
            self.selected_sorter.clone(),
            Message::SorterSelected,
        )
        .text_size(20)
        // .width(Length::FillPortion(2))
        .style(BaronyModManagerUiStyles);

        let pick_list_full = Column::new()
            .spacing(5)
            .push(pick_list_label)
            .push(pick_list);

        let above_middle = Row::new()
            .push(pick_list_full)
            .push(show_only_installed)
            .spacing(20)
            .align_items(Align::Center);
        let middle_section = Column::new().height(Length::Fill);

        let all_content = Column::new()
            .spacing(20)
            .push(top)
            .push(above_middle)
            .push(middle_section)
            .push(api_key_section);

        Container::new(all_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(BaronyModManagerUiStyles)
            .into()
    }
}
