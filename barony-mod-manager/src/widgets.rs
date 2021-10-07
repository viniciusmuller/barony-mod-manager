use std::{
    fmt::{self, Display},
    hash::{Hash, Hasher},
};

use image::{DynamicImage, ImageBuffer, Rgba};

use crate::data::{self, BaronyMod, SteamApiResponse, SteamWorkshopMod, SteamWorkshopTag};

#[derive(Clone, Debug)]
pub enum Message {
    ApiKeyInputChanged(String),
    ModSearchInputChanged(String),
    BaronyDirectoryPathChanged(String),
    ToggleHiddenApiKeyInput(bool),
    ToggleShowOnlyInstalled(bool),
    TotalModsNumber(u64),
    TagSelected(PickableTag),
    FilterSelected(Filter),
    ModFetched(BaronyMod),
    ModImageFetched(String, ImageBuffer<Rgba<u8>, Vec<u8>>),
    EventOccurred(iced_native::Event),
    SorterSelected(Sorter),
    ErrorHappened(String),
    ButtonWasPressed,
    NoOp,
}

#[derive(Debug, Clone, Hash)]
pub enum PickableTag {
    Some(SteamWorkshopTag),
    None,
}

impl Display for PickableTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PickableTag::Some(tag) => tag.display_name.clone(),
                PickableTag::None => "None".to_string(),
            }
        )
    }
}

// impl Hash for PickableTag {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         match self {
//             Some(tag) => self.tag.hash(state),
//             _ => None
//         }
//     }
// }

impl Default for PickableTag {
    fn default() -> PickableTag {
        PickableTag::None
    }
}

impl PartialEq for PickableTag {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Eq for PickableTag {}

/// Possible fields that can be used to sort the mods
#[derive(Clone, Debug)]
pub enum Sorter {
    VoteScore,
    Views,
    Subscribed,
    Updated,
    Created,
    None,
}

impl Sorter {
    pub const ALL: [Sorter; 6] = [
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
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
impl Eq for Sorter {}

impl Default for Sorter {
    fn default() -> Sorter {
        Sorter::None
    }
}

/// Possible fields that can be used to sort the mods
#[derive(Clone, Debug)]
pub enum Filter {
    Active,
    Inactive,
    Downloaded,
    NonDownloaded,
    None,
}

impl Filter {
    pub const ALL: [Filter; 5] = [
        Filter::Active,
        Filter::Inactive,
        Filter::Downloaded,
        Filter::NonDownloaded,
        Filter::None,
    ];
}

impl Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Filter::Active => "Active",
                Filter::Inactive => "Inactive",
                Filter::Downloaded => "Downloaded",
                Filter::NonDownloaded => "Non Downloaded",
                Filter::None => "None",
            }
        )
    }
}

impl PartialEq for Filter {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
impl Eq for Filter {}

impl Default for Filter {
    fn default() -> Filter {
        Filter::None
    }
}
