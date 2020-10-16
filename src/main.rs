use std::{io::Write, path::PathBuf};

use clap::{Arg, ArgMatches};
use crossterm::{
    cursor::Show,
    event::{KeyCode, KeyEvent, KeyModifiers},
    QueueableCommand,
};
use entry::Entry;
use writer::TiwiWriter;

mod entry;
mod writer;

fn main() -> anyhow::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let result = run();
    crossterm::terminal::disable_raw_mode()?;
    result
}

fn app() -> clap::App<'static, 'static> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    clap::App::new("tiwi")
        .version(VERSION)
        .before_help("Tiwi is an interactive application which keeps track of timestamped events.")
        .arg(
            Arg::with_name("UTC")
                .short("u")
                .long("utc")
                .help("Appends events with UTC timestamps"),
        )
        .arg(
            Arg::with_name("FILE")
                .required(false)
                .index(1)
                .help("Appends events to the given file"),
        )
}

pub fn init() -> ArgMatches<'static> {
    app().get_matches()
}

fn run() -> anyhow::Result<()> {
    let args = init();
    let utc = args.is_present("UTC");

    let mut writer = if let Some(path) = args.value_of("FILE") {
        let path = PathBuf::from(path);
        TiwiWriter::stdout_and_file(path.as_path())?
    } else {
        TiwiWriter::stdout()
    };

    let mut new_entry: Option<Entry> = None;
    let mut completed_entry: Option<Entry> = None;

    while let Ok(event) = crossterm::event::read() {
        let action = match event {
            crossterm::event::Event::Key(key) => handle_key(key),
            _ => Action::Continue,
        };

        if let Action::Continue = action {
            continue;
        }

        let entry = create_entry(&mut new_entry, utc);

        if let Some(ref mut completed) = completed_entry {
            completed.before(entry);
            writer.flush(&completed)?;
            completed_entry = None;
        }

        match action {
            Action::Append(ch) => {
                entry.push(ch);
                writer.update(entry)?;
            }
            Action::CompleteEntry => {
                writer.complete(&entry)?;

                completed_entry = new_entry;
                new_entry = None;
            }
            Action::Backspace => {
                entry.backspace();
                writer.update(entry)?;
            }
            Action::Terminate => {
                writer.flush(entry)?;

                std::io::stdout().queue(Show)?;
                std::io::stdout().flush()?;

                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn create_entry<'c>(current: &'c mut Option<Entry>, utc: bool) -> &'c mut Entry {
    if let None = current {
        let new = if utc { Entry::utc() } else { Entry::local() };

        *current = Some(new)
    }

    current.as_mut().unwrap()
}

#[derive(Debug)]
enum Action {
    Append(char),
    Backspace,
    CompleteEntry,
    Continue,
    Terminate,
}

// fn mark_timestamp(
//     last_entry: &mut Option<Instant>,
//     new_entry: &mut bool,
//     stdout: &mut TiwiWriter,
//     utc: bool,
// ) -> anyhow::Result<()> {
//     if !*new_entry {
//         return Ok(());
//     }

//     let entry: Entry<Utc> = Entry::new();
//     if utc {
//         let date = Utc::now();
//         stdout.write(date.to_rfc3339_opts(SecondsFormat::Secs, true).as_bytes())?;
//     } else {
//         let date = Local::now();
//         stdout.write(date.to_rfc3339_opts(SecondsFormat::Secs, true).as_bytes())?;
//     }

//     stdout.write("\t".as_bytes())?;

//     if let Some(last) = last_entry {
//         let now = Instant::now();
//         let duration = now.duration_since(last.clone());
//         let seconds = duration.as_secs_f32();

//         let display = if seconds > 60.0 {
//             let minutes = (seconds / 60.0).round() as usize;
//             format!("{}m", minutes)
//         } else {
//             let seconds = seconds.round() as usize;
//             format!("{}s", seconds)
//         };

//         stdout.write(display.as_bytes())?;
//     } else {
//         stdout.write("--".as_bytes())?;
//     }

//     stdout.write("\t".as_bytes())?;

//     *last_entry = Some(Instant::now());
//     *new_entry = false;

//     Ok(())
// }

fn handle_key(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Enter => Action::CompleteEntry,
        KeyCode::Backspace => Action::Backspace,
        // KeyCode::Left => {}
        // KeyCode::Right => {}
        // KeyCode::Up => {}
        // KeyCode::Down => {}
        // KeyCode::Home => {}
        // KeyCode::End => {}
        // KeyCode::PageUp => {}
        // KeyCode::PageDown => {}
        KeyCode::Tab => Action::Append('\t'),
        // KeyCode::BackTab => {}
        // KeyCode::Delete => {}
        // KeyCode::Insert => {}
        // KeyCode::F(_) => {}
        KeyCode::Char(ch) => {
            if ch == '\n' {
                Action::CompleteEntry
            } else if key.modifiers == KeyModifiers::CONTROL && (ch == 'c' || ch == 'x') {
                Action::Terminate
            } else {
                Action::Append(ch)
            }
        }
        KeyCode::Null => Action::Terminate,
        KeyCode::Esc => Action::Terminate,
        _ => Action::Continue,
    }
}
