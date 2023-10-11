#![allow(clippy::too_many_lines)]

mod coin;
mod graphics;
mod mine;
mod randomizer;
mod snake;

use graphics::{
    clear,
    cursor::{self, DetectCursorPos},
    raw::{IntoRawMode, RawTerminal},
};
use std::{
    borrow::BorrowMut,
    io::{stdin, stdout, Read, StdoutLock, Write},
};

const GAMES: &[&str; 4] = &["coin game", "mine sweeper", "snake", "empty"];

fn main() {
    let stdin = stdin();
    let mut stdin_lock = stdin.lock();

    let mut stdout = stdout().lock().into_raw_mode().unwrap();

    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    stdout.flush().unwrap();

    let mut selected = 0;

    show_list(&mut stdout, selected);

    for c in stdin_lock.borrow_mut().bytes() {
        match c.unwrap() {
            b'w' => {
                if selected > 0 {
                    selected = selected.saturating_sub(1);
                }
            }
            b's' => {
                if selected < GAMES.len() - 1 {
                    selected += 1;
                }
            }
            b' ' => match GAMES[selected] {
                "coin game" => {
                    coin::coin_game(&mut stdin_lock, &mut stdout);
                    break;
                }
                "mine sweeper" => {
                    mine::mine_sweeper(&mut stdin_lock, &mut stdout);
                    break;
                }
                "snake" => {
                    snake::snake_game();
                    break;
                }
                _ => (),
            },
            b'q' => break,
            _ => (),
        }

        show_list(&mut stdout, selected);

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", cursor::Show).unwrap();
}

fn show_list(stdout: &mut RawTerminal<StdoutLock>, selected: usize) {
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

    for (i, game) in GAMES.iter().enumerate() {
        if i == selected {
            writeln!(stdout, "> {game}").unwrap();
        } else {
            writeln!(stdout, "  {game}").unwrap();
        }
        let (_, y) = stdout.cursor_pos().unwrap();
        write!(stdout, "{}", cursor::Goto(1, y + 1)).unwrap();
    }
}
