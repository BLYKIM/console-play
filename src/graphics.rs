pub use termion::{async_stdin, clear, color, cursor, event, input, raw, style};

pub const MINE: &str = "*";
pub const FLAGGED: &str = "F";
pub const CONCEALED: &str = "▒";
pub const BORDER: &str = "#";
pub const PLAYER: &str = "&";
pub const COIN: &str = "o";
pub const VERTICAL_SNAKE_BODY: &str = "║";
pub const HORIZONTAL_SNAKE_BODY: &str = "═";
pub const SNAKE_HEAD: &str = "@";

pub const GAME_START_PROMPT: &str = "Press 'space' to start";
pub const GAME_OVER: &str = "Game Over. Press 'r' to reset, Press 'q' to exit";
