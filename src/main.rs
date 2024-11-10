use std::{io::stdout, thread};

use api::app::{Application, Tui};
use flexi_logger::{FileSpec, Logger, TS_DASHES_BLANK_COLONS_DOT_BLANK};
use log::{error, info};
use ratatui::{
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::CrosstermBackend,
    Terminal,
};
use shared::error::{AppError, AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    if let Err(e) = bootstrap().await {
        error!("{e}");
        return Err(e);
    }

    Ok(())
}

pub async fn bootstrap() -> AppResult<()> {
    logger_init()?;

    api::system::supported()?;
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let tui = Box::new(Tui::default());
    let app_result = tui.run(&mut terminal).await;

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    app_result
}

fn logger_init() -> AppResult<()> {
    use flexi_logger::{Cleanup, Criterion, Naming};

    Logger::try_with_str("info")?
        .log_to_file(FileSpec::default().use_timestamp(true).directory("./log"))
        .rotate(
            Criterion::Age(flexi_logger::Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(7),
        )
        .format_for_files(|w, now, record| -> std::io::Result<()> {
            write!(
                w,
                "{} {} [Thread {}] Message:{}",
                record.level(),
                now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK),
                thread::current().name().unwrap_or("unnamed"),
                &record.args()
            )
        })
        .start()?;
    Ok(())
}
