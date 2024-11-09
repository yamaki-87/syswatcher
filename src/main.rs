use std::io::stdout;

use flexi_logger::{FileSpec, Logger};
use log::info;
use ratatui::{crossterm::{terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand}, prelude::CrosstermBackend, Terminal};
use shared::error::{AppError, AppResult};
use api::app::{Application, Tui};



#[tokio::main]
async fn main()->AppResult<()>{
    bootstrap().await
}

pub async fn bootstrap()->AppResult<()>{
    logger_init()?;

    info!("test");
    api::system::supported()?;
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let tui = Box::new(Tui::default());
    let app_result=tui.run(&mut terminal).await;

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    app_result
}

fn logger_init()->AppResult<()>{
    use flexi_logger::{Criterion,Naming,Cleanup};

    Logger::try_with_str("info")?
        .log_to_file(
            FileSpec::default()
                .directory("./log")
        )
        .rotate(Criterion::Age(flexi_logger::Age::Day), Naming::Timestamps, Cleanup::KeepLogFiles(7))
        .start()?;
    Ok(())
}