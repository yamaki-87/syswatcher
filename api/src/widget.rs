use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{palette::tailwind, Stylize},
    text::{Line, Text},
    widgets::{Paragraph, StatefulWidget, StatefulWidgetRef, Widget, Wrap},
};
use strum::{Display, EnumIter, FromRepr};
use tui_scrollview::{ScrollView, ScrollViewState};

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

#[derive(Debug, Default)]
pub struct ProcessTab  {
    pub process_text:Text<'static>,
}

impl ProcessTab {
    fn new() -> Self {
        Self {
            process_text:"test".into(),
        }
    }
}

impl StatefulWidgetRef for ProcessTab {
    type State = ScrollViewState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        const SCROLLVIEW_HEIGHT: u16 = 80;
        let mut scroll_view = ScrollView::new((area.width - 1, SCROLLVIEW_HEIGHT).into());
        scroll_view.render_widget(
            Paragraph::new(self.process_text.clone())
                .white()
                .on_green()
                .wrap(Wrap::default()),
            Rect::new(0, 0, area.width-1, SCROLLVIEW_HEIGHT),
        );

        scroll_view.render(area, buf, state);
    }
}
