use arsenal_terminal::app::{run_app, ArsenalApp};
use crossterm::{
    // event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    Terminal
};
use arg::Args;
#[derive(Args, Debug)]
struct MyArgs {
    #[arg(short, long, default_value="false")]
    ///Verbose mode
    _verbose: bool,

    #[arg(short, long)]
    ///To store path
    settings: String,
}
use home;


pub mod arsenal_terminal;
pub mod arsenal_objects;
pub mod misc;


fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments
    let args: MyArgs = arg::parse_args();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let mut app = ArsenalApp::new(100);

    // Try loading settings
    if args.settings != "" {  // Settings by option in args
        _ = app.load_settings(args.settings);
    } else {  // Settings present in user home directory (~/.config/cyberarsenal/settings.toml)
        match home::home_dir() {
            Some(path) if !path.as_os_str().is_empty() => {
                match path.to_str() {
                    Some(path_home_dir) => {
                        let settings_path = format!("{}/.config/cyberarsenal/settings.toml", path_home_dir);
                        _ = app.load_settings(settings_path);
                    },
                    None => {}
                };
            },
            _ => {},
        }
    }

    let res = run_app(&mut terminal, app, tick_rate);
    if let Err(err) = res {
        println!("{:?}", err)
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
