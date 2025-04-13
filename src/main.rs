mod app;
mod args;
mod keys;
mod tabs;
mod ui;

use crate::{app::App, args::process_cmdline};
use std::{
    cell::RefCell,
    io::{self, Stdout},
    panic,
    path::Path,
    process,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    let app_start = Instant::now();

    let cliargs = process_cmdline()?;

    let key_config = KeyConfig::init()
        .map_err(|e| eprintln!("KeyConfig loading error: {e}"))
        .unwrap_or_default();
    let theme = Theme::init(&cliargs.theme);

    setup_terminal()?;
    defer! {
        shutdown_terminal();
    }

    set_panic_handlers()?;

    let mut repo_path = cliargs.repo_path;
    let mut terminal = start_terminal(io::stdout(), &repo_path)?;
    let input = Input::new();

    let updater = if cliargs.notify_watcher {
        Updater::NotifyWatcher
    } else {
        Updater::Ticker
    };

    loop {
        let quit_state = run_app(
            app_start,
            repo_path.clone(),
            theme.clone(),
            key_config.clone(),
            &input,
            updater,
            &mut terminal,
        )?;

        match quit_state {
            QuitState::OpenSubmodule(p) => {
                repo_path = p;
            }
            _ => break,
        }
    }

    Ok(())
}
