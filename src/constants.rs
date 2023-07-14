use crossterm::event::KeyCode;

pub const DATA_SOURCE_NAME: &str = "todos.json";
pub const DEV_DATA_FOLDER: &str = ".dev_data_source";
pub const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub const TICK: &str = "   [x] ";
pub const SELECTED_TICK: &str = ">> [x] ";
pub const SELECTED_UNTICK: &str = ">> [ ] ";
pub const UNTICKED: &str = "   [ ] ";
pub const START_SCREEN: &str = "START_SCREEN";

pub const DEFAULT_DELTE: KeyCode = KeyCode::Char('d');
pub const DEFAULT_ADD: KeyCode = KeyCode::Char('a');
pub const DEFAULT_QUIT: KeyCode = KeyCode::Char('q');
pub const DEFAULT_SAVE: KeyCode = KeyCode::Char('S');

pub const DEFAULT_UP: KeyCode = KeyCode::Up;
pub const DEFAULT_DOWN: KeyCode = KeyCode::Down;
pub const DEFAULT_ENTER: KeyCode = KeyCode::Enter;
pub const DEFAULT_ESC: KeyCode = KeyCode::Esc;

pub const APP_NAME: &str = env!("CARGO_BIN_NAME");

pub mod dev {
    use once_cell::sync::Lazy;

    pub const LIPSUM: &str = "LIPSUM";
    pub const LIPSUM_ITEMS: &str = "LIPSUM_ITEMS";
    pub const LIPSUM_MIN_WORD: &str = "LIPSUM_MIN_WORD";
    pub const LIPSUM_MAX_WORD: &str = "LIPSUM_MAX_WORD";

    pub const USE_USER_FOLDER_ENV: &str = "USE_USER_FOLDER";

    pub static USE_USER_FOLDER: Lazy<bool> = Lazy::new(|| {
        const INTENDED_DEFAULT: bool = false;

        std::env::var(USE_USER_FOLDER_ENV)
            .ok()
            .map(|to_parse| to_parse.parse().unwrap_or(INTENDED_DEFAULT))
            .unwrap_or(INTENDED_DEFAULT)
    });
}
