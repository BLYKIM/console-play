use crate::graphics::{clear, cursor, raw::RawTerminal, BORDER, COIN, PLAYER};
use rand::Rng;
use std::{
    io::{Read, StdoutLock, Write},
    time,
};

const MANUAL_POS: u16 = 23;
const PLAY_TIME: u64 = 60;
const COIN_TIME: u64 = 2;
const MAIN_POINT: u16 = 2;
const SIZE: u16 = 20;

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
        clear::All,
        cursor::Goto(MANUAL_POS, MAIN_POINT),
        cursor::Hide
    )
    .unwrap();

    let mut player = Position {
        x: MAIN_POINT,
        y: MAIN_POINT,
    };
    let mut score = 0;

    // coin
    let mut rng = rand::thread_rng();
    let mut coin = Position {
        x: rng.gen_range(MAIN_POINT..SIZE),
        y: rng.gen_range(MAIN_POINT..SIZE),
    };

    draw_border(stdout);
    draw_coin(stdout, &coin);

    let start_time = time::Instant::now();
    let mut coin_time = start_time;

    stdout.flush().unwrap();

    // move character
    for c in stdin.bytes() {
        if time_exceeded(start_time, PLAY_TIME) {
            break;
        }
        if time_exceeded(coin_time, COIN_TIME) {
            coin_time = time::Instant::now();
            clear_coin(stdout, &coin);

            coin.x = rng.gen_range(MAIN_POINT..SIZE);
            coin.y = rng.gen_range(MAIN_POINT..SIZE);
            draw_coin(stdout, &coin);
        }

        if player.x == coin.x && player.y == coin.y {
            score += 1;
            coin_time = time::Instant::now();

            coin.x = rng.gen_range(MAIN_POINT..SIZE);
            coin.y = rng.gen_range(MAIN_POINT..SIZE);
            draw_coin(stdout, &coin);
        } else {
            clear_player(stdout, &player);
        }

        match c.unwrap() {
            b'q' => break,
            b'a' => {
                if player.x > MAIN_POINT {
                    player.x -= 1;
                }
            }
            b'd' => {
                if player.x < SIZE {
                    player.x += 1;
                }
            }
            b'w' => {
                if player.y > MAIN_POINT {
                    player.y -= 1;
                }
            }
            b's' => {
                if player.y < SIZE {
                    player.y += 1;
                }
            }
            _ => {}
        }

        if player.x == coin.x && player.y == coin.y {
            score += 1;
            coin_time = time::Instant::now();

            coin.x = rng.gen_range(MAIN_POINT..SIZE);
            coin.y = rng.gen_range(MAIN_POINT..SIZE);
            draw_coin(stdout, &coin);
        }

        draw_player(stdout, &player);

        write!(
            stdout,
            "{}Score: {}",
            cursor::Goto(MANUAL_POS, MAIN_POINT + 3),
            score
        )
        .unwrap();

        stdout.flush().unwrap();
    }

    write!(stdout, "{}Score: {}{}", clear::All, score, cursor::Show).unwrap();
}

fn draw_border(stdout: &mut RawTerminal<StdoutLock>) {
    for i in (MAIN_POINT - 1)..(SIZE + 2) {
        write!(
            stdout,
            "{}{}{}{}",
            cursor::Goto(i, MAIN_POINT - 1),
            BORDER,
            cursor::Goto(i, SIZE + 1),
            BORDER
        )
        .unwrap();

        if i < SIZE {
            write!(
                stdout,
                "{}{}{}{}",
                cursor::Goto(MAIN_POINT - 1, i + 1),
                BORDER,
                cursor::Goto(SIZE + 1, i + 1),
                BORDER
            )
            .unwrap();
        }
    }
}

fn draw_coin(stdout: &mut RawTerminal<StdoutLock>, coin: &Position) {
    write!(stdout, "{}{}", cursor::Goto(coin.x, coin.y), COIN).unwrap();
}

/// Clear the current coin
fn clear_coin(stdout: &mut RawTerminal<StdoutLock>, coin: &Position) {
    write!(stdout, "{} ", cursor::Goto(coin.x, coin.y)).unwrap();
}

/// Clear the current character
fn clear_player(stdout: &mut RawTerminal<StdoutLock>, player: &Position) {
    write!(stdout, "{} ", cursor::Goto(player.x, player.y)).unwrap();
}

/// Write the '&' character
fn draw_player(stdout: &mut RawTerminal<StdoutLock>, player: &Position) {
    write!(stdout, "{}{}", cursor::Goto(player.x, player.y), PLAYER).unwrap();
}

fn time_exceeded(start_time: time::Instant, limit: u64) -> bool {
    time::Instant::now().duration_since(start_time) > time::Duration::from_secs(limit)
}
