#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::unused_io_amount
)]

use crate::graphics::{clear, color, cursor, style, BORDER, RACER_TYPE, RACE_NUMBER_PROMPT};
use rand::{seq::SliceRandom, Rng};
use std::{
    io::{Read, StdoutLock, Write},
    time::{Duration, Instant},
};
use termion::{async_stdin, raw::RawTerminal};

struct Racer {
    racer_type: char,
    name: String,
    pos: u16,
    speed: u16,
    line_num: u16,
}

impl Racer {
    fn new(name: String, line_num: u16) -> Self {
        let mut rng = rand::thread_rng();
        let racer_type = *RACER_TYPE.choose(&mut rng).unwrap();

        Self {
            racer_type,
            name,
            pos: 1,
            speed: 1,
            line_num,
        }
    }

    fn run(&mut self) {
        let mut rng = rand::thread_rng();
        self.speed = rng.gen_range(1..8);
        self.pos += self.speed;
    }
}

/// The game state.
struct Game<R, W> {
    /// The play area width.
    width: usize,
    /// The play lines.
    lines: u8,
    /// Standard input.
    stdin: R,
    /// Standard output.
    stdout: W,
    /// Racer
    racers: Vec<Racer>,
}

impl<R: Read, W: Write> Game<R, W> {
    /// Start the game loop.
    ///
    /// This will listen to events and do the appropriate actions.
    fn start(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();
        let num_players = self.game_start_prompt();
        self.reset(num_players);

        let mut last_update = Instant::now();
        let mut async_stdin = async_stdin().bytes();

        loop {
            let input = async_stdin.next();

            if let Some(Ok(b'q')) = input {
                return;
            }

            if last_update.elapsed() > Duration::from_millis(100) {
                self.clear_player();

                self.move_player();

                self.draw_player();
                write!(self.stdout, "{}", style::Reset).unwrap();
                self.stdout.flush().unwrap();

                last_update = Instant::now();
            } else {
                std::thread::sleep(Duration::from_millis(10));
            }

            let (winner, result) = self.check_game_over();
            if result {
                if self.game_over(&winner) {
                    self.reset(num_players);
                    continue;
                }
                return;
            }
        }
    }

    /// Reset the game.
    ///
    /// This will display the starting play area.
    fn reset(&mut self, num_players: u8) {
        write!(self.stdout, "{}{}", clear::All, style::Reset).unwrap();

        self.draw_walls(num_players as u16);

        for i in 0..num_players {
            self.racers
                .push(Racer::new((i + 1).to_string(), (i as u16 * 2) + 1));
        }
    }

    fn clear_player(&mut self) {
        for racer in &self.racers {
            write!(
                self.stdout,
                "{} ",
                cursor::Goto(racer.pos + 1, racer.line_num + 1)
            )
            .unwrap();
        }
    }

    fn move_player(&mut self) {
        for racer in &mut self.racers {
            racer.run();
        }
    }

    #[allow(unused_assignments)]
    fn game_start_prompt(&mut self) -> u8 {
        write!(self.stdout, "{}{}", cursor::Goto(1, 1), RACE_NUMBER_PROMPT).unwrap();
        self.stdout.flush().unwrap();
        loop {
            let mut buf = [0];
            self.stdin.read(&mut buf).unwrap();

            match buf[0] {
                b' ' => return 4,
                b'2'..=b'9' => return buf[0] - b'0',
                _ => {}
            }
        }
    }

    fn draw_horizontal_line(&mut self, chr: &str, width: u16) {
        for _ in 0..width {
            self.stdout.write(chr.as_bytes()).unwrap();
        }
    }

    /// Draws the player.
    fn draw_player(&mut self) {
        for racer in &self.racers {
            write!(
                self.stdout,
                "{}",
                cursor::Goto(racer.pos + 1, racer.line_num + 1)
            )
            .unwrap();
            self.stdout
                .write(racer.racer_type.to_string().as_bytes())
                .unwrap();
        }
    }

    /// Draws the game walls.
    fn draw_walls(&mut self, lines: u16) {
        let width: u16 = self.width as u16;

        write!(self.stdout, "{}", color::Fg(color::Red)).unwrap();

        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
        self.draw_horizontal_line(BORDER, width);

        for y in 1..lines {
            write!(self.stdout, "{}", cursor::Goto(1, y * 2 + 1)).unwrap();
            self.draw_horizontal_line(BORDER, width);
        }

        write!(self.stdout, "{}", cursor::Goto(1, lines * 2 + 1)).unwrap();
        self.draw_horizontal_line(BORDER, width);

        write!(self.stdout, "{}", color::Fg(color::Reset)).unwrap();
    }

    fn check_game_over(&mut self) -> (String, bool) {
        let mut result = Vec::new();
        let mut rank = String::new();
        let mut is_end = false;

        for racer in &self.racers {
            if racer.pos >= self.width as u16 {
                for racer in &self.racers {
                    result.push((racer.name.clone(), racer.pos));
                }
                is_end = true;
                break;
            }
        }
        result.sort_by(|a, b| b.1.cmp(&a.1));

        for r in result {
            rank.push_str(&r.0);
            rank.push('\n');
        }
        (rank, is_end)
    }

    fn game_over(&mut self, winners: &str) -> bool {
        write!(
            self.stdout,
            "{}Ranking: {}, Press 'q' to exit.",
            cursor::Goto(20, 15),
            winners
        )
        .unwrap();
        write!(
            self.stdout,
            "{}",
            cursor::Goto((self.width as u16 / 2) - 2, self.lines as u16 / 2 + 1)
        )
        .unwrap();
        self.stdout.flush().unwrap();

        loop {
            // Repeatedly read a single byte.
            let mut buf = [0];
            if self.stdin.read(&mut buf).is_ok() && buf[0] == b'q' {
                return false;
            }
        }
    }
}

/// Initializes the game.
fn init<W: Write, R: Read>(mut stdout: W, stdin: R, width: usize) {
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    let mut game = Game {
        width,
        lines: 0,
        stdin,
        stdout,
        racers: Vec::new(),
    };

    game.start();

    write!(
        game.stdout,
        "{}{}{}",
        clear::All,
        style::Reset,
        cursor::Goto(1, 1)
    )
    .unwrap();
    game.stdout.flush().unwrap();
}

#[allow(clippy::module_name_repetitions)]
pub fn coffee_race<R: Read>(stdin: &mut R, stdout: &mut RawTerminal<StdoutLock>) {
    let width = match termion::terminal_size() {
        Ok((w, _)) => w - 5,
        Err(_) => 200,
    } as usize;
    init(stdout, stdin, width);
}
