mod data;
mod file;
mod parse;
mod utils;

use std::error::Error;
use std::io::Stdout;
use std::slice::Chunks;
use std::time::Duration;
use std::{env, future, thread};

use crossterm::terminal;
use data::{CrossTerminal, Secret};
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Text};
use tui::widgets::Paragraph;
use tui::Frame;
use utils::{show_current_data, show_furture_data};

use crate::file::{get_secret_file_path, load_secret, write_json_file};
use crate::parse::parse_arguments;
use crate::utils::show_data;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

pub type DynResult = Result<(), Box<dyn std::error::Error>>;
pub type TerminalFrame<'a> = tui::Frame<'a, CrosstermBackend<Stdout>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut secret: Secret = load_secret().await?;
    let args: Vec<String> = env::args().collect();

    parse_arguments(&args, &mut secret)?;

    let file_path_str = get_secret_file_path().to_string_lossy().to_string();
    write_json_file(&file_path_str, &secret).await?;
    show_data(&secret).await?;
    let mut terminal = init_terminal()?;
    let current_weather_str = show_current_data(&secret).await?;
    let furture_weather_str = show_furture_data(&secret).await?;
    terminal.draw(|f: &mut Frame<'_, CrosstermBackend<Stdout>>| {
        let current_text_size = Rect::new(0, 0, f.size().width, f.size().height / 4);

        let current_lines = Text::from((&current_weather_str).as_str());
        let current_weather = Paragraph::new(current_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Current weather"),
        );
        f.render_widget(current_weather, current_text_size);
        let future_text_size =
            Rect::new(0, f.size().height / 4, f.size().width, f.size().height / 2);
        let furture_lines = Text::from((&furture_weather_str).as_str());
        let future_weather = Paragraph::new(furture_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Future weather"),
        );
        f.render_widget(future_weather, future_text_size);
    })?;
    thread::sleep(Duration::from_millis(5000));
    close_terminal(terminal)?;
    Ok(())
}

fn init_terminal() -> Result<CrossTerminal, Box<dyn Error>> {
    let mut stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    //必须先执行EnableMouseCapture后面才能支持鼠标事件
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn close_terminal(mut terminal: CrossTerminal) -> DynResult {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::event::DisableMouseCapture,
        crossterm::terminal::LeaveAlternateScreen
    )?;
    Ok(())
}
