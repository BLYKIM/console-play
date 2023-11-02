pub use termion::{async_stdin, clear, color, cursor, event, input, raw, style};

pub const MAIN_INTRO: &str = "
C O N S O L E - P L A Y\n\r
            B L Y\n\r
Select Game with 'w' 's' then press 'space'\n\r
";
pub const MINE: &str = "*";
pub const FLAGGED: &str = "F";
pub const CONCEALED: &str = "▒";
pub const BORDER: &str = "#";
pub const PLAYER: &str = "&";
pub const COIN: &str = "o";
pub const VERTICAL_SNAKE_BODY: &str = "║";
pub const HORIZONTAL_SNAKE_BODY: &str = "═";
pub const SNAKE_HEAD: &str = "@";
pub const RACER_TYPE: [char; 10] = ['🐥', '🐶', '🐷', '@', '😃', '💩', '🐌', '🦀', '🌜', '👺'];

pub const GAME_START_PROMPT: &str = "Press 'space' to start";
pub const RACE_NUMBER_PROMPT: &str = "Enter the number of players. [2 - 9]";
pub const GAME_OVER: &str = "Game Over. Press 'q' to exit";
