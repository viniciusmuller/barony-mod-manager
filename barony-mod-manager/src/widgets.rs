use std::fmt::{self, Display};

use crate::data::{BaronyMod, SteamWorkshopMod};

#[derive(Clone, Debug)]
pub enum Message {
    // UI related events
    ModSearchInputChanged(String),
    BaronyDirectoryPathChanged(String),
    TagSelected(PickableTag),
    FilterSelected(Filter),
    SorterSelected(Sorter),
    SortingStrategySelected(SortingStrategy),
    CloseRequested,
    LoadMods,
    ErrorHappened(String),

    // Application inner workings' events
    ModsFetched(Vec<SteamWorkshopMod>),
    ModBuilt(BaronyMod),
    DownloadMod(String),
    PreparingModDownload(String, String),
    ModDownloadReady(String, String),
    ModDownloaded(String),
    RemoveMod(String),

    // Misc
    NoOp,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum PickableTag {
    Some(String),
    None,
}

impl Display for PickableTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PickableTag::Some(tag) => tag.clone(),
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

/// Possible fields that can be used to sort the mods
#[derive(Clone, Debug)]
pub enum Sorter {
    VoteScore,
    Views,
    Size,
    Updated,
    Created,
    None,
}

impl Sorter {
    pub const ALL: [Sorter; 6] = [
        Sorter::VoteScore,
        Sorter::Views,
        Sorter::Size,
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Filter {
    Downloaded,
    NonDownloaded,
    Downloading,
    None,
}

impl Filter {
    pub const ALL: [Filter; 4] = [
        Filter::Downloading,
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
                Filter::Downloaded => "Downloaded",
                Filter::NonDownloaded => "Non Downloaded",
                Filter::Downloading => "Downloading",
                Filter::None => "None",
            }
        )
    }
}

impl Default for Filter {
    fn default() -> Filter {
        Filter::None
    }
}

/// Possible fields that can be used to sort the mods
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SortingStrategy {
    Ascending,
    Descending,
}

impl SortingStrategy {
    pub const ALL: [SortingStrategy; 2] = [SortingStrategy::Descending, SortingStrategy::Ascending];
}

impl Display for SortingStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SortingStrategy::Ascending => "Ascending",
                SortingStrategy::Descending => "Descending",
            }
        )
    }
}

impl Default for SortingStrategy {
    fn default() -> SortingStrategy {
        SortingStrategy::Descending
    }
}
