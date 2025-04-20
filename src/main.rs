mod app;
mod args;
// mod input;
mod components;
mod keys;
mod tabs;
mod ui;
mod string_utils;
mod strings;

mod cmdbar;
use crate::{app::App, args::process_cmdline};
use anyhow::{anyhow, bail, Result};
use std::{
    cell::RefCell,
    io::{self, Stdout},
    panic,
    path::Path,
    process,
    time::{Duration, Instant},
};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
// use input::{Input, InputEvent, InputState};
use app::QuitState;
use keys::KeyConfig;
use ratatui::backend::CrosstermBackend;
use scopeguard::defer;
use ui::style::Theme;

type Terminal = ratatui::Terminal<CrosstermBackend<io::Stdout>>;

#[derive(Clone)]
pub enum QueueEvent {
    Tick,
    Notify,
    SpinnerUpdate,
    // AsyncEvent(AsyncNotification),
    // InputEvent(InputEvent),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntaxHighlightProgress {
    Progress,
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncAppNotification {
    ///
    SyntaxHighlighting(SyntaxHighlightProgress),
}

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum AsyncNotification {
//     ///
//     App(AsyncAppNotification),
//     ///
//     Git(AsyncGitNotification),
// }

// #[derive(Clone, Copy, PartialEq)]
// enum Updater {
//     Ticker,
//     NotifyWatcher,
// }

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

    let mut terminal = start_terminal(io::stdout())?;
    // let input = Input::new();

    // let updater = if cliargs.notify_watcher {
    //     Updater::NotifyWatcher
    // } else {
    //     Updater::Ticker
    // };

    loop {
        let quit_state = run_app(
            app_start,
            // repo_path.clone(),
            theme.clone(),
            key_config.clone(),
            // &input,
            // updater,
            &mut terminal,
        )?;

        break;
    }

    Ok(())
}

fn setup_terminal() -> Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

fn shutdown_terminal() {
    let leave_screen = io::stdout().execute(LeaveAlternateScreen).map(|_f| ());

    if let Err(e) = leave_screen {
        eprintln!("leave_screen failed:\n{e}");
    }

    let leave_raw_mode = disable_raw_mode();

    if let Err(e) = leave_raw_mode {
        eprintln!("leave_raw_mode failed:\n{e}");
    }
}
fn start_terminal(buf: Stdout) -> Result<Terminal> {
    let mut backend = CrosstermBackend::new(buf);
    backend.execute(crossterm::terminal::SetTitle(format!("gitui ()",)))?;

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
};

fn run_app(
    app_start: Instant,
    theme: Theme,
    key_config: KeyConfig,
    // input: &Input,
    // updater: Updater,
    terminal: &mut Terminal,
) -> Result<QuitState, anyhow::Error> {
    // let (tx_git, rx_git) = unbounded();
    // let (tx_app, rx_app) = unbounded();

    // let rx_input = input.receiver();

    // let (rx_ticker, rx_watcher) = match updater {
    //     Updater::NotifyWatcher => {
    //         let repo_watcher = RepoWatcher::new(repo_work_dir(&repo)?.as_str());

    //         (never(), repo_watcher.receiver())
    //     }
    //     Updater::Ticker => (tick(TICK_INTERVAL), never()),
    // };

    // let spinner_ticker = tick(SPINNER_INTERVAL);

    let mut app = App::new(
        // RefCell::new(repo),
        // tx_git,
        // tx_app,
        // input.clone(),
        theme,
        key_config,
    )?;

    // let mut spinner = Spinner::default();
    // let mut first_update = true;

    // log::trace!("app start: {} ms", app_start.elapsed().as_millis());

    loop {
        // let event = if first_update {
        //     first_update = false;
        //     QueueEvent::Notify
        // } else {
        //     select_event(
        //         &rx_input,
        //         &rx_git,
        //         &rx_app,
        //         &rx_ticker,
        //         &rx_watcher,
        //         &spinner_ticker,
        //     )?
        // };

        {
            // if matches!(event, QueueEvent::SpinnerUpdate) {
            //     spinner.update();
            //     spinner.draw(terminal)?;
            //     continue;
            // }

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        break Ok(QuitState::None);
                    }
                }
            }
            // scope_time!("loop");

            // match event {
            //     QueueEvent::InputEvent(ev) => {
            //         if matches!(ev, InputEvent::State(InputState::Polling)) {
            //             //Note: external ed closed, we need to re-hide cursor
            //             terminal.hide_cursor()?;
            //         }
            //         app.event(ev)?;
            //     }
            //     QueueEvent::Tick | QueueEvent::Notify => {
            //         app.update()?;
            //     }
            //     QueueEvent::AsyncEvent(ev) => {
            //         if !matches!(
            //             ev,
            //             AsyncNotification::Git(AsyncGitNotification::FinishUnchanged)
            //         ) {
            //             app.update_async(ev)?;
            //         }
            //     }
            //     QueueEvent::SpinnerUpdate => unreachable!(),
            // }

            draw(terminal, &app)?;

            // spinner.set_state(app.any_work_pending());
            // spinner.draw(terminal)?;

            // if app.is_quit() {
            //     break;
            // }
        }
    }

    // Ok(app.quit_state())
}
// fn select_event(
//     rx_input: &Receiver<InputEvent>,
//     rx_git: &Receiver<AsyncGitNotification>,
//     rx_app: &Receiver<AsyncAppNotification>,
//     rx_ticker: &Receiver<Instant>,
//     rx_notify: &Receiver<()>,
//     rx_spinner: &Receiver<Instant>,
// ) -> Result<QueueEvent> {
//     let mut sel = Select::new();

//     sel.recv(rx_input);
//     sel.recv(rx_git);
//     sel.recv(rx_app);
//     sel.recv(rx_ticker);
//     sel.recv(rx_notify);
//     sel.recv(rx_spinner);

//     let oper = sel.select();
//     let index = oper.index();

//     let ev = match index {
//         0 => oper.recv(rx_input).map(QueueEvent::InputEvent),
//         1 => oper
//             .recv(rx_git)
//             .map(|e| QueueEvent::AsyncEvent(AsyncNotification::Git(e))),
//         2 => oper
//             .recv(rx_app)
//             .map(|e| QueueEvent::AsyncEvent(AsyncNotification::App(e))),
//         3 => oper.recv(rx_ticker).map(|_| QueueEvent::Notify),
//         4 => oper.recv(rx_notify).map(|()| QueueEvent::Notify),
//         5 => oper.recv(rx_spinner).map(|_| QueueEvent::SpinnerUpdate),
//         _ => bail!("unknown select source"),
//     }?;

//     Ok(ev)
// }

fn draw(terminal: &mut Terminal, app: &App) -> io::Result<()> {
    // if app.requires_redraw() {
    //     terminal.clear()?;
    // }

    terminal.draw(|f| {
        if let Err(e) = app.draw(f) {
            log::error!("failed to draw: {:?}", e);
        }
    })?;

    Ok(())
}
