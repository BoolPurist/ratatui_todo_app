use crossterm::event::KeyCode;

pub const DATA_SOURCE_NAME: &str = "todos.json";
pub const DEV_DATA_FOLDER: &str = ".dev_data_source";
pub const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub const LINE_SPACE_BETWEEN_TODOS: usize = 1;
pub const TICK: &str = "   [x] ";
pub const SELECTED_TICK: &str = ">> [x] ";
pub const SELECTED_UNTICK: &str = ">> [ ] ";
pub const UNTICKED: &str = "   [ ] ";
pub const START_SCREEN: &str = "START_SCREEN";

pub const DEFAULT_DELTE: KeyCode = KeyCode::Char('d');
pub const DEFAULT_ADD: KeyCode = KeyCode::Char('a');
pub const DEFAULT_BACK: KeyCode = KeyCode::Char('b');
pub const DEFAULT_QUIT: KeyCode = KeyCode::Char('q');
pub const DEFAULT_SAVE: KeyCode = KeyCode::Char('S');

pub const DEFAULT_UP: KeyCode = KeyCode::Up;
pub const DEFAULT_DOWN: KeyCode = KeyCode::Down;
pub const DEFAULT_ENTER: KeyCode = KeyCode::Enter;
pub const DEFAULT_ESC: KeyCode = KeyCode::Esc;
