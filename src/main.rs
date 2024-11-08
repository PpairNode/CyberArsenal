use arsenal_terminal::app::ArsenalApp;
use std::{error::Error, io::Stdout};
use arg::Args;

#[cfg(not(feature = "nogui"))]
use std::io;
#[cfg(not(feature = "nogui"))]
use arsenal_terminal::app::run_app;
#[cfg(not(feature = "nogui"))]
use std::time::Duration;
#[cfg(not(feature = "nogui"))]
use tui::{backend::CrosstermBackend, Terminal};
#[cfg(not(feature = "nogui"))]
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};

pub mod arsenal_terminal;
pub mod arsenal_objects;
pub mod misc;



#[derive(Args, Debug)]
struct MyArgs {
    #[arg(short, long, default_value="false")]
    ///Verbose mode
    _verbose: bool,

    #[arg(short, long)]
    ///To store path
    settings: String,
}


#[cfg(not(feature = "nogui"))]
fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    // Restore terminal
    #[cfg(not(feature = "nogui"))]
    disable_raw_mode()?;
    #[cfg(not(feature = "nogui"))]
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    #[cfg(not(feature = "nogui"))]
    terminal.show_cursor()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments
    let args: MyArgs = arg::parse_args();

    // Setup terminal
    #[cfg(not(feature = "nogui"))]
    enable_raw_mode()?;
    #[cfg(not(feature = "nogui"))]
    let mut stdout = io::stdout();
    #[cfg(not(feature = "nogui"))]
    execute!(stdout, EnterAlternateScreen)?;
    #[cfg(not(feature = "nogui"))]
    let backend = CrosstermBackend::new(stdout);
    #[cfg(not(feature = "nogui"))]
    let mut terminal = Terminal::new(backend)?;

    // Create application
    #[cfg(not(feature = "nogui"))]
    let tick_rate = Duration::from_millis(250);
    let mut app = ArsenalApp::new(100);

    // Loading settings in application or error
    if let Err(e) = app.load_settings(args.settings) {
        eprintln!("load_settings failed, error={}", e);
        #[cfg(feature = "nogui")]
        return Ok(())
    };

    // Run application
    #[cfg(not(feature = "nogui"))] {
        let res = run_app(&mut terminal, app, tick_rate);
        if let Err(err) = res {
            println!("{:?}", err)
        }
    }

    // Restore terminal
    #[cfg(not(feature = "nogui"))]
    let _ = restore_terminal(&mut terminal);

    Ok(())
}
