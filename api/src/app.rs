use async_trait::async_trait;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    prelude::*,
    style::{Modifier, Style, Stylize},
    symbols::border::{self},
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Gauge, Padding, Paragraph, Tabs, Widget, *,
    },
    DefaultTerminal,
};


use shared::error::{AppError, AppResult};
use strum::IntoEnumIterator;
use tui_scrollview::{ScrollView, ScrollViewState};

use crate::{system::{SysData, SysInfo}, widget::SelectedTab};

macro_rules! title_block {
    ($title:expr)=>{
        title_block!($title,2)
    };
    ($title:expr,$padding:expr) => {
        {
            let title = Title::from($title.bold().blue()).alignment(ratatui::layout::Alignment::Center);
            Block::bordered()
                .title(title)
                .border_set(border::EMPTY)
                .padding(Padding::vertical($padding))
        }
    };
}

#[derive(Default)]
pub struct Tui {
    sysinfos: SysInfo,
    sysdata: SysData,
    state: AppState,
    selected_tab: SelectedTab,
    scrollview_state: ScrollViewState,
}
#[derive(Default,PartialEq, Eq,)]
pub enum AppState {
    #[default]
    RUNNING,
    QUIT,
}

#[async_trait]
pub trait Application {
    async fn run(mut self,terminal:&mut DefaultTerminal)->AppResult<()>;
    fn handle_events(&mut self)->AppResult<()>;
    fn handle_key_event(&mut self,key_event:KeyEvent);
    fn update(&mut self);
}

#[async_trait]
impl Application for Tui {
    async fn run(mut self, terminal: &mut DefaultTerminal) -> AppResult<()> {
        while self.is_running() {
            self.handle_events()?;

            self.update();
            terminal
                .draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> AppResult<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => self.refresh(),
            KeyCode::Left => self.privious_tab(),
            KeyCode::Tab | KeyCode::Right => self.next_tab(),
            KeyCode::Char('G') if self.selected_tab == SelectedTab::Process => {
                self.scrollview_state.scroll_to_bottom()
            }
            KeyCode::Char('g') if self.selected_tab == SelectedTab::Process => {
                self.scrollview_state.scroll_to_top()
            }
            KeyCode::Up if self.selected_tab == SelectedTab::Process => {
                self.scrollview_state.scroll_up()
            }
            KeyCode::Down if self.selected_tab == SelectedTab::Process => {
                self.scrollview_state.scroll_down()
            }
            _ => {}
        }
    }

    fn update(&mut self) {
        if self.state != AppState::RUNNING {
            return;
        }

        self.sysinfos.refresh_all();
    }
}


impl Tui {
    fn draw_os_info(&self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(Line::from(" OS Info ".blue().bold()));

        let block = Block::bordered()
            .title(title.alignment(ratatui::layout::Alignment::Center))
            .border_set(border::THICK);

        let os_info = Text::from(vec![
            Line::from(vec!["OS: ".into(), self.sysdata.get_os_long_ver().green()]),
            Line::from(vec!["HOST: ".into(), self.sysdata.get_host().green()]),
            Line::from(vec![
                "Uptime: ".into(),
                format!("{}", self.sysdata.get_uptime()).green(),
            ]),
            Line::from(vec![
                "Boot Time: ".into(),
                format!("{}", self.sysdata.get_boot_time()).green(),
            ]),
            Line::from(vec![
                "CPU Architecture: ".into(),
                self.sysdata.get_cpu_arch().green(),
            ]),
            Line::from(vec![
                "Kernel Version: ".into(),
                self.sysdata.get_kernel_ver().green(),
            ]),
        ]);

        Paragraph::new(os_info)
            .left_aligned()
            .block(block)
            .render(area, buf);
    }

    fn draw_bottom(&self, area: Rect, buf: &mut Buffer) {
        let instructions = Title::from(Line::from(vec![
            " Quit ".into(),
            "<Q>".red().bold(),
            " Refresh ".into(),
            "<R>".red().bold(),
        ]));

        Block::bordered()
            .title(
                instructions
                    .alignment(ratatui::layout::Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::EMPTY)
            .render(area, buf);
    }

    fn draw_mem_info(&self, area: Rect, buf: &mut Buffer) {
        let mem = self.sysinfos.get_memory();

        Gauge::default()
            .block(title_block!(" Memory Usage ",1))
            .gauge_style(
                Style::default()
                    .fg(ratatui::style::Color::Blue)
                    .bg(ratatui::style::Color::White)
                    .add_modifier(Modifier::ITALIC),
            )
            .ratio(mem / 100.)
            .use_unicode(true)
            .label(format!("{:.2}%", mem))
            .render(area, buf);
    }

    fn render_cpu_info(&self, area: Rect, buf: &mut Buffer) {
        let cpu = self.sysinfos.get_cpu() as f64;

        Gauge::default()
            .block(title_block!(" CPU Usage "))
            .gauge_style(
                Style::default()
                    .fg(ratatui::style::Color::Blue)
                    .bg(ratatui::style::Color::White),
            )
            .ratio(cpu / 100.)
            .use_unicode(true)
            .label(format!("{:.2}%", cpu))
            .render(area, buf);
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let selected_tab_idx = self.selected_tab as usize;
        Tabs::new(titles)
            .select(selected_tab_idx)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    fn render_processes_scrollview(&self, buf: &mut Buffer) {
        let area = buf.area;

        let [numbers, widgets] =
            Layout::horizontal([Constraint::Length(5), Constraint::Fill(1)]).areas(area);

        let line = Text::from(
            self.sysinfos
                .get_processes()
                .iter()
                .map(|(pid, process)| format!("{} {:?}\n", pid.as_u32(), process.name()))
                .collect::<String>(),
        );

        Paragraph::new(line)
            .block(
                Block::bordered()
                    .border_set(border::THICK)
                    .title(" Process ")
                    .title_alignment(Alignment::Center),
            )
            .wrap(Wrap { trim: false })
            .render(widgets, buf);
    }
}

//汎用
impl Tui {
    fn exit(&mut self) {
        self.state = AppState::QUIT;
    }

    fn refresh(&mut self) {
        self.sysinfos = SysInfo::default();
        self.sysdata = SysData::default();
    }

    fn is_running(&self)->bool{
        self.state == AppState::RUNNING
    }
}

impl Tui {
    fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    fn privious_tab(&mut self) {
        self.selected_tab = self.selected_tab.privious();
    }
}

impl Widget for &mut Tui {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self.selected_tab {
            SelectedTab::Main => {
                let [tab_footer, main, bottom] = Layout::vertical([
                    Constraint::Percentage(7),
                    Constraint::Percentage(84),
                    Constraint::Percentage(9),
                ])
                .areas(area);

                let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(main);
                let [mem_gauge_area, cpu_gauge_area, _, _] =
                    Layout::vertical([Constraint::Ratio(1, 4); 4]).areas(left);

                self.render_tabs(tab_footer, buf);
                self.draw_mem_info(mem_gauge_area, buf);
                self.render_cpu_info(cpu_gauge_area, buf);
                self.draw_os_info(right, buf);
                self.draw_bottom(bottom, buf);
            }
            SelectedTab::Process => {
                let mut scrollview = ScrollView::new((area.width - 2, 80).into());
                self.render_processes_scrollview(scrollview.buf_mut());
                scrollview.render(area, buf, &mut self.scrollview_state);
            }
        }
    }
}

fn title_block(title: &str) -> Block {
    let title = Title::from(title.bold().blue()).alignment(ratatui::layout::Alignment::Center);
    Block::bordered()
        .title(title)
        .border_set(border::EMPTY)
        .padding(Padding::vertical(2))
}
