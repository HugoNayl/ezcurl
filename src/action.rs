#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Action {
    Move(Direction),
    MoveCursor(Direction),
    MoveCursorToStart,
    MoveCursorToEnd,
    NextField,
    PreviousField,
    NextPanel,
    EnterPanel,
    ExitPanel,
    EnterInsert,
    ExitInsert,
    ToggleHistory,
    Leader,
    CancelLeader,
    Activate,
    Close,
    SendRequest,
    InsertChar(char),
    InsertNewline,
    Backspace,
    Delete,
    Quit,
}
