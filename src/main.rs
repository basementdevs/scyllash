use clap::Parser;
use scyllash::args::Command;
use scyllash::{app::App, args::Arguments, tui};

fn main() -> color_eyre::Result<()> {
    let args = Arguments::parse();
    
    match &args.command {
        Some(Command::Run { .. }) => {
            run_interface()
        }
        None => {
            run_interface()
        }
    }
}

fn run_interface() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    if let Err(err) = tui::restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }
    app_result
}
