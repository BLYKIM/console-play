#![allow(clippy::too_many_lines)]
use rand::Rng;
use std::io::{stdin, stdout, Write};
use std::time;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const MANUAL_POS: u16 = 23;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // init
    write!(
        stdout,
        "{}{}q to exit. Use arrow keys to move the character.{}",
        termion::clear::All,
        termion::cursor::Goto(MANUAL_POS, 2),
        termion::cursor::Hide
    )
    .unwrap();

    let mut x = 2;
    let mut y = 2;
    let mut score = 0;

    // coin
    let mut rng = rand::thread_rng();
    let mut target_x: u16 = rng.gen_range(2..20);
    let mut target_y: u16 = rng.gen_range(2..20);

    // Draw the border
    for i in 1..22 {
        write!(
            stdout,
            "{}#{}#",
            termion::cursor::Goto(i, 1),
            termion::cursor::Goto(i, 21)
        )
        .unwrap();
        if i < 20 {
            write!(
                stdout,
                "{}#{}#",
                termion::cursor::Goto(1, i + 1),
                termion::cursor::Goto(21, i + 1)
            )
            .unwrap();
        }
    }

    write!(stdout, "{}o", termion::cursor::Goto(target_x, target_y)).unwrap();

    let start_time = time::Instant::now();
    let mut coin_time = start_time;
    // move character
    for c in stdin.keys() {
        if time::Instant::now().duration_since(start_time) > time::Duration::from_secs(60) {
            break;
        }
        if time::Instant::now().duration_since(coin_time) > time::Duration::from_secs(2) {
            coin_time = time::Instant::now();
            write!(stdout, "{} ", termion::cursor::Goto(target_x, target_y)).unwrap(); // Clear the current coin

            target_x = rng.gen_range(2..20);
            target_y = rng.gen_range(2..20);
            write!(stdout, "{}o", termion::cursor::Goto(target_x, target_y)).unwrap();
        }

        if x == target_x && y == target_y {
            score += 1;
            coin_time = time::Instant::now();

            target_x = rng.gen_range(2..20);
            target_y = rng.gen_range(2..20);
            write!(stdout, "{}o", termion::cursor::Goto(target_x, target_y)).unwrap();
        } else {
            write!(stdout, "{} ", termion::cursor::Goto(x, y)).unwrap(); // Clear the current character
        }
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Left => {
                if x > 2 {
                    x -= 1;
                }
            }
            Key::Right => {
                if x < 20 {
                    x += 1;
                }
            }
            Key::Up => {
                if y > 2 {
                    y -= 1;
                }
            }
            Key::Down => {
                if y < 20 {
                    y += 1;
                }
            }
            _ => {}
        }

        if x == target_x && y == target_y {
            score += 1;
            coin_time = time::Instant::now();
            target_x = rng.gen_range(2..20);
            target_y = rng.gen_range(2..20);
            write!(stdout, "{}o", termion::cursor::Goto(target_x, target_y)).unwrap();
        }

        write!(stdout, "{}&", termion::cursor::Goto(x, y)).unwrap(); // Write the '&' character

        write!(
            stdout,
            "{}Score: {score}",
            termion::cursor::Goto(MANUAL_POS, 5)
        )
        .unwrap();
        // write!(
        //     stdout,
        //     "{}Debug: x:{x}, y:{y}, target_x: {target_x}, target_y: {target_y}",
        //     termion::cursor::Goto(MANUAL_POS, 5)
        // )
        // .unwrap();
        stdout.flush().unwrap();
    }

    write!(
        stdout,
        "{}Score: {}{}",
        termion::clear::All,
        score,
        termion::cursor::Show
    )
    .unwrap();
}
