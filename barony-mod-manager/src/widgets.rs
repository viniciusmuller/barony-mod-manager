use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

use crate::data::{BaronyMod, SteamWorkshopTag};

#[derive(Clone, Debug)]
pub enum Message {
    // UI related events
    ApiKeyInputChanged(String),
    ModSearchInputChanged(String),
    BaronyDirectoryPathChanged(String),
    ToggleHiddenApiKeyInput(bool),
    ToggleShowOnlyInstalled(bool),
    TagSelected(PickableTag),
    FilterSelected(Filter),
    SorterSelected(Sorter),
    CloseRequested,
    LoadMods,
    ErrorHappened(String),

    // Application inner workings' events
    TotalModsNumber(u64),
    ModFetched(BaronyMod),
    DownloadMod(String),
    PreparingModDownload(String, String),
    ModDownloadReady(String, String),
    ModDownloaded(String),
    RemoveMod(String),
    ToggleActivateMod(String, bool),

    // Misc
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

impl Ord for PickableTag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Greater)
    }
}

impl PartialOrd for PickableTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            PickableTag::Some(tag) => match other {
                PickableTag::Some(other_tag) => tag.tag.cmp(&other_tag.tag),
                _ => Ordering::Less,
            },
            _ => Ordering::Less,
        })
    }
}

/// Possible fields that can be used to sort the mods
#[derive(Clone, Debug)]
pub enum Sorter {
    VoteScore,
    Views,
    Size,
    Subscribed,
    Updated,
    Created,
    None,
}

impl Sorter {
    pub const ALL: [Sorter; 7] = [
        Sorter::VoteScore,
        Sorter::Views,
        Sorter::Size,
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
                Sorter::Size => "Size",
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

// #[derive(Clone, Debug)]
// TODO: Have two view style: `Fancy` (the current) and `Simple` which will
// look like a table view without images or description.
pub enum ViewStyle {
    Fancy,
    Simple,
}

impl PartialEq for ViewStyle {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
impl Eq for ViewStyle {}

impl Default for ViewStyle {
    fn default() -> ViewStyle {
        ViewStyle::Fancy
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
