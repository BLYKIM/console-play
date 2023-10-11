#![allow(clippy::unused_io_amount, clippy::unused_self)]

use crate::{
    graphics::{
        clear, color, cursor, event::Key, input::TermRead, raw::RawTerminal, style, BORDER,
        CONCEALED, FLAGGED, MINE,
    },
    randomizer::Randomizer,
};
use std::io::{Read, StdoutLock, Write};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    mine: bool,
    revealed: bool,
    observed: bool,
    flagged: bool,
}

struct MineSweeper<R, W: Write> {
    width: u16,
    grid: Box<[Cell]>,
    x: u16,
    y: u16,
    rand: Randomizer,
    score: u16,
    stdout: W,
    stdin: R,
}

#[allow(clippy::module_name_repetitions)]
pub fn mine_sweeper<R: Read>(stdin: &mut R, stdout: &mut RawTerminal<StdoutLock>) {
    write!(stdout, "{}", clear::All).unwrap();
    // init
    write!(
        stdout,
        "{}{}q to exit. Use arrow keys to move and 'f' to flag, 'space' to select.{}",
        clear::All,
        cursor::Goto(24, 2),
        cursor::Hide
    )
    .unwrap();

    init(stdout, stdin);
}

fn init<W: Write, R: Read>(stdout: W, stdin: R) {
    let mut game = MineSweeper {
        width: 20,
        x: 0,
        y: 0,
        rand: Randomizer::new(0),
        grid: vec![
            Cell {
                mine: false,
                revealed: false,
                observed: false,
                flagged: false,
            };
            400
        ]
        .into_boxed_slice(),
        score: 0,
        stdin: stdin.keys(),
        stdout,
    };

    game.reset();
    game.start();
}

impl<R, W: Write> Drop for MineSweeper<R, W> {
    fn drop(&mut self) {
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(1, 1)
        )
        .unwrap();
    }
}

impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> MineSweeper<R, W> {
    fn pos(&self, x: u16, y: u16) -> usize {
        y as usize * self.width as usize + x as usize
    }

    fn read_cell(&mut self, c: usize) {
        if !self.grid[c].observed {
            self.grid[c].mine = self.rand.read_u8() % 5 == 0;
            self.grid[c].observed = true;
        }
    }

    fn get(&mut self, x: u16, y: u16) -> Cell {
        let pos = self.pos(x, y);

        self.read_cell(pos);
        self.grid[pos]
    }

    fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        let pos = self.pos(x, y);

        self.read_cell(pos);
        &mut self.grid[pos]
    }

    fn start(&mut self) {
        let mut first_click = true;
        loop {
            let b = self.stdin.next().unwrap().unwrap();
            if let Key::Char(c) = b {
                self.rand.write_u8(c as u8);
            }

            match b {
                Key::Char('a') | Key::Left => self.x = self.move_left(self.x),
                Key::Char('d') | Key::Right => self.x = self.move_right(self.x),
                Key::Char('w') | Key::Up => self.y = self.move_up(self.y),
                Key::Char('s') | Key::Down => self.y = self.move_down(self.y),
                Key::Char(' ') => {
                    let (x, y) = (self.x, self.y);

                    if first_click {
                        for &(x, y) in &self.adjacent(x, y) {
                            self.get_mut(x, y).mine = false;
                        }
                        self.get_mut(x, y).mine = false;
                        first_click = false;
                    }

                    if self.get(x, y).mine {
                        self.reveal_all();
                        write!(
                            self.stdout,
                            "{}{}{}{}{}",
                            cursor::Goto(x + 2, y + 2),
                            color::Bg(color::Red),
                            color::Fg(color::Black),
                            MINE,
                            style::Reset
                        )
                        .unwrap();
                        self.game_over();
                        return;
                    }

                    if !self.get(x, y).revealed {
                        self.score += 1;
                    }

                    self.reveal(x, y);
                    self.print_score();
                }
                Key::Char('f') => {
                    let (x, y) = (self.x, self.y);
                    self.toggle_flag(x, y);
                }
                Key::Char('q') => return,
                _ => {}
            }

            write!(self.stdout, "{}", cursor::Goto(self.x + 2, self.y + 2),).unwrap();
            write!(self.stdout, "{}", cursor::Show).unwrap();
            self.stdout.flush().unwrap();
        }
    }

    fn set_flag(&mut self, x: u16, y: u16) {
        if !self.get(x, y).revealed {
            self.stdout.write(FLAGGED.as_bytes()).unwrap();
            self.get_mut(x, y).flagged = true;
        }
    }

    fn remove_flag(&mut self, x: u16, y: u16) {
        self.stdout.write(CONCEALED.as_bytes()).unwrap();
        self.get_mut(x, y).flagged = false;
    }

    fn toggle_flag(&mut self, x: u16, y: u16) {
        if self.get(x, y).flagged {
            self.remove_flag(x, y);
        } else {
            self.set_flag(x, y);
        }
    }

    fn reset(&mut self) {
        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();

        // Draw top border
        for _ in 0..(self.width + 2) {
            self.stdout.write(BORDER.as_bytes()).unwrap();
        }
        self.stdout.write(b"\n\r").unwrap();

        // Conceal all the cells
        for _ in 0..self.height() {
            self.stdout.write(BORDER.as_bytes()).unwrap();

            for _ in 0..self.width {
                self.stdout.write(CONCEALED.as_bytes()).unwrap();
            }
            self.stdout.write(BORDER.as_bytes()).unwrap();
            self.stdout.write(b"\n\r").unwrap();
        }

        // Draw bottom border
        for _ in 0..(self.width + 2) {
            self.stdout.write(BORDER.as_bytes()).unwrap();
        }

        write!(self.stdout, "{}", cursor::Goto(self.x + 2, self.y + 2),).unwrap();

        self.stdout.flush().unwrap();

        // reset the grid
        for i in 0..self.grid.len() {
            // Fill it with random, concealed fields
            self.grid[i] = Cell {
                mine: false,
                revealed: false,
                observed: false,
                flagged: false,
            };

            self.score = 0;
        }
    }

    fn val(&mut self, x: u16, y: u16) -> u8 {
        let mut res = 0;
        for &(x, y) in &self.adjacent(x, y) {
            res += u8::from(self.get(x, y).mine);
        }
        res
    }

    fn reveal(&mut self, x: u16, y: u16) {
        let v = self.val(x, y);

        self.get_mut(x, y).revealed = true;

        write!(self.stdout, "{}", cursor::Goto(x + 2, y + 2)).unwrap();

        if v == 0 {
            // If the cell is free, simply put a space on the position
            self.stdout.write(b" ").unwrap();

            for &(x, y) in &self.adjacent(x, y) {
                if !self.get(x, y).revealed && !self.get(x, y).mine {
                    self.reveal(x, y);
                }
            }
        } else {
            // The cell is not free, Print the value instead.
            self.stdout.write(&[b'0' + v]).unwrap();
        }
    }

    fn print_score(&mut self) {
        let height = self.height();
        write!(self.stdout, "{}", cursor::Goto(24, height + 2)).unwrap();
        self.stdout
            .write(self.score.to_string().as_bytes())
            .unwrap();
    }

    fn reveal_all(&mut self) {
        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();

        for y in 0..self.height() {
            for x in 0..self.width {
                write!(self.stdout, "{}", cursor::Goto(x + 2, y + 2)).unwrap();
                if self.get(x, y).mine {
                    self.stdout.write(MINE.as_bytes()).unwrap();
                }
            }
        }
    }

    fn game_over(&mut self) {
        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();

        self.stdout.write(b"Game Over. press q to exit.").unwrap();
        self.stdout.flush().unwrap();

        loop {
            if let Key::Char('q') = self.stdin.next().unwrap().unwrap() {
                return;
            }
        }
    }

    fn adjacent(&self, x: u16, y: u16) -> Vec<(u16, u16)> {
        let mut cells = Vec::new();

        if let Some(left) = self.left(x) {
            cells.push((left, y));
            if let Some(up) = self.up(y) {
                cells.push((left, up));
            }
            if let Some(down) = self.down(y) {
                cells.push((left, down));
            }
        }

        if let Some(right) = self.right(x) {
            cells.push((right, y));
            if let Some(up) = self.up(y) {
                cells.push((right, up));
            }
            if let Some(down) = self.down(y) {
                cells.push((right, down));
            }
        }

        if let Some(up) = self.up(y) {
            cells.push((x, up));
        }
        if let Some(down) = self.down(y) {
            cells.push((x, down));
        }

        cells
    }

    fn height(&self) -> u16 {
        u16::try_from(self.grid.len() / self.width as usize).unwrap()
    }

    fn up(&self, y: u16) -> Option<u16> {
        if y == 0 {
            None
        } else {
            Some(y - 1)
        }
    }

    fn down(&self, y: u16) -> Option<u16> {
        if y + 1 == self.height() {
            None
        } else {
            Some(y + 1)
        }
    }

    fn left(&self, x: u16) -> Option<u16> {
        if x == 0 {
            None
        } else {
            Some(x - 1)
        }
    }

    fn right(&self, x: u16) -> Option<u16> {
        if x + 1 == self.width {
            None
        } else {
            Some(x + 1)
        }
    }

    fn move_up(&self, y: u16) -> u16 {
        if y == 0 {
            0
        } else {
            y - 1
        }
    }

    fn move_down(&self, y: u16) -> u16 {
        if y + 1 == self.height() {
            y
        } else {
            y + 1
        }
    }

    fn move_left(&self, x: u16) -> u16 {
        if x == 0 {
            0
        } else {
            x - 1
        }
    }

    fn move_right(&self, x: u16) -> u16 {
        if x + 1 == self.width {
            x
        } else {
            x + 1
        }
    }
}
