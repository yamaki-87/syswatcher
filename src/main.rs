use std::io::stdout;

use ratatui::{crossterm::{terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand}, prelude::CrosstermBackend, Terminal};
use shared::error::{AppError, AppResult};
use api::app::{Application, Tui};



#[tokio::main]
async fn main()->AppResult<()>{
    bootstrap().await
}

pub async fn bootstrap()->AppResult<()>{
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let tui = Box::new(Tui::default());
    let app_result=tui.run(&mut terminal).await;

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    app_result
}
