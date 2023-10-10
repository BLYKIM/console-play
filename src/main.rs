#![allow(clippy::too_many_lines)]

mod coin;

use std::borrow::BorrowMut;
use std::io::{stdin, stdout, StdoutLock, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

const GAMES: &[&str; 3] = &["coin game", "empty", "empty"];

fn main() {
    let stdin = stdin();
    let mut stdin_lock = stdin.lock();

    let mut stdout = stdout().lock().into_raw_mode().unwrap();

    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    stdout.flush().unwrap();

    let mut selected = 0;

    show_list(&mut stdout, selected);

    for c in stdin_lock.borrow_mut().keys() {
        match c.unwrap() {
            Key::Up => {
                if selected > 0 {
                    selected = selected.saturating_sub(1);
                }
            }
            Key::Down => {
                if selected < GAMES.len() - 1 {
                    selected += 1;
                }
            }
            Key::Char('\n') => match GAMES[selected] {
                "coin game" => {
                    coin::coin_game(&mut stdin_lock, &mut stdout);
                    break;
                }
                "empty" => break,
                _ => (),
            },
            Key::Char('q') => break,
            _ => (),
        }

        show_list(&mut stdout, selected);

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn show_list(stdout: &mut RawTerminal<StdoutLock>, selected: usize) {
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    for (i, game) in GAMES.iter().enumerate() {
        if i == selected {
            writeln!(stdout, "> {game}").unwrap();
        } else {
            writeln!(stdout, "  {game}").unwrap();
        }
        let (_, y) = stdout.cursor_pos().unwrap();
        write!(stdout, "{}", termion::cursor::Goto(1, y + 1)).unwrap();
    }
}
