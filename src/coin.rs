use rand::Rng;
use std::io::{Read, StdoutLock, Write};
use std::time;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;

const MANUAL_POS: u16 = 23;
const PLAY_TIME: u64 = 10;
const COIN_TIME: u64 = 2;

struct Position {
    x: u16,
    y: u16,
}

#[allow(clippy::module_name_repetitions)]
pub fn coin_game<R: Read>(stdin: &mut R, stdout: &mut RawTerminal<StdoutLock>) {
    // init
    write!(
        stdout,
        "{}{}q to exit. Use arrow keys to move the character.{}",
        termion::clear::All,
        termion::cursor::Goto(MANUAL_POS, 2),
        termion::cursor::Hide
    )
    .unwrap();

    let mut player = Position { x: 2, y: 2 };
    let mut score = 0;

    // coin
    let mut rng = rand::thread_rng();
    let mut coin = Position {
        x: rng.gen_range(2..20),
        y: rng.gen_range(2..20),
    };

    draw_border(stdout);
    draw_coin(stdout, &coin);

    let start_time = time::Instant::now();
    let mut coin_time = start_time;

    stdout.flush().unwrap();

    // move character
    for c in stdin.keys() {
        if time_exceeded(start_time, PLAY_TIME) {
            break;
        }
        if time_exceeded(coin_time, COIN_TIME) {
            coin_time = time::Instant::now();
            clear_coin(stdout, &coin);

            coin.x = rng.gen_range(2..20);
            coin.y = rng.gen_range(2..20);
            draw_coin(stdout, &coin);
        }

        if player.x == coin.x && player.y == coin.y {
            score += 1;
            coin_time = time::Instant::now();

            coin.x = rng.gen_range(2..20);
            coin.y = rng.gen_range(2..20);
            draw_coin(stdout, &coin);
        } else {
            clear_player(stdout, &player);
        }

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Left => {
                if player.x > 2 {
                    player.x -= 1;
                }
            }
            Key::Right => {
                if player.x < 20 {
                    player.x += 1;
                }
            }
            Key::Up => {
                if player.y > 2 {
                    player.y -= 1;
                }
            }
            Key::Down => {
                if player.y < 20 {
                    player.y += 1;
                }
            }
            _ => {}
        }

        if player.x == coin.x && player.y == coin.y {
            score += 1;
            coin_time = time::Instant::now();

            coin.x = rng.gen_range(2..20);
            coin.y = rng.gen_range(2..20);
            draw_coin(stdout, &coin);
        }

        draw_player(stdout, &player);

        write!(
            stdout,
            "{}Score: {}",
            termion::cursor::Goto(MANUAL_POS, 5),
            score
        )
        .unwrap();

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

fn draw_border(stdout: &mut RawTerminal<StdoutLock>) {
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
}

fn draw_coin(stdout: &mut RawTerminal<StdoutLock>, coin: &Position) {
    write!(stdout, "{}o", termion::cursor::Goto(coin.x, coin.y)).unwrap();
}

/// Clear the current coin
fn clear_coin(stdout: &mut RawTerminal<StdoutLock>, coin: &Position) {
    write!(stdout, "{} ", termion::cursor::Goto(coin.x, coin.y)).unwrap();
}

/// Clear the current character
fn clear_player(stdout: &mut RawTerminal<StdoutLock>, player: &Position) {
    write!(stdout, "{} ", termion::cursor::Goto(player.x, player.y)).unwrap();
}

/// Write the '&' character
fn draw_player(stdout: &mut RawTerminal<StdoutLock>, player: &Position) {
    write!(stdout, "{}&", termion::cursor::Goto(player.x, player.y)).unwrap();
}

fn time_exceeded(start_time: time::Instant, limit: u64) -> bool {
    time::Instant::now().duration_since(start_time) > time::Duration::from_secs(limit)
}
