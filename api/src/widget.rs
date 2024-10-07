

use ratatui::{style::{palette::tailwind, Stylize}, text::Line};
use strum::{EnumIter, FromRepr,Display};

#[derive(Default, Display, FromRepr, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Main")]
    Main,
    #[strum(to_string = "Process")]
    Process,
}

impl SelectedTab {
    pub fn privious(self) -> Self {
        let current_idx = self as usize;
        let privious_idx = current_idx.saturating_sub(1);
        Self::from_repr(privious_idx).unwrap_or(self)
    }

    pub fn next(self) -> Self {
        let current_idx = self as usize;
        let next_idx = current_idx.saturating_add(1);
        Self::from_repr(next_idx).unwrap_or(self)
    }
}

impl SelectedTab {
    pub fn title(self) -> Line<'static> {
        format!(" {self} ").fg(tailwind::SLATE.c200).into()
    }
}
