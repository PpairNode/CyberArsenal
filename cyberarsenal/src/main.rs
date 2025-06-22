use arsenal_terminal::app::ArsenalApp;
use std::{error::Error, io::Stdout};
use arg::Args;
use tracing::error;

use std::io;
use arsenal_terminal::app::run_app;
use std::time::Duration;
use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};

pub mod arsenal_terminal;
pub mod arsenal_objects;
pub mod misc;
use misc::logs::init_tracing;


#[derive(Args, Debug)]
struct MyArgs {
    #[arg(short, long, default_value="false")]
    ///Verbose mode
    _verbose: bool,

    #[arg(short, long)]
    ///To store path
    settings: String,
}


fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    init_tracing()?;

    // Parse arguments
    let args: MyArgs = arg::parse_args();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create application
    let tick_rate = Duration::from_millis(250);
    let mut app = ArsenalApp::new(100);

    // Loading settings in application or error
    if let Err(e) = app.load_settings(args.settings) {
        error!("load_settings failed, error={}", e);
        let _ = restore_terminal(&mut terminal);
        return Ok(())
    };

    // Run application
    
    let res = run_app(&mut terminal, app, tick_rate);
    if let Err(err) = res {
        println!("{:?}", err)
    }

    // Restore terminal
    let _ = restore_terminal(&mut terminal);

    Ok(())
}
