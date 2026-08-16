use crate::{
    action::{Action, Direction},
    app::{AppMode, Panel},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn map_key(key: KeyEvent, mode: AppMode, panel: Panel, leader_pending: bool) -> Option<Action> {
    if leader_pending {
        return match key.code {
            KeyCode::Char('e' | 'E') => Some(Action::ToggleHistory),
            _ => Some(Action::CancelLeader),
        };
    }

    if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return Some(Action::SendRequest);
    }

    match mode {
        AppMode::SelectPanel => match key.code {
            KeyCode::Enter => Some(Action::EnterPanel),
            KeyCode::Char('h') => Some(Action::Move(Direction::Left)),
            KeyCode::Char('j') => Some(Action::Move(Direction::Down)),
            KeyCode::Char('k') => Some(Action::Move(Direction::Up)),
            KeyCode::Char('l') => Some(Action::Move(Direction::Right)),
            KeyCode::Left => Some(Action::MoveCursor(Direction::Left)),
            KeyCode::Right => Some(Action::MoveCursor(Direction::Right)),
            KeyCode::Up => Some(Action::MoveCursor(Direction::Up)),
            KeyCode::Down => Some(Action::MoveCursor(Direction::Down)),
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char(' ') => Some(Action::Leader),
            KeyCode::Tab => Some(Action::NextPanel),
            _ => None,
        },
        AppMode::Normal => match key.code {
            KeyCode::Esc => Some(Action::ExitPanel),
            KeyCode::Char('h') => Some(Action::Move(Direction::Left)),
            KeyCode::Char('j') => Some(Action::Move(Direction::Down)),
            KeyCode::Char('k') => Some(Action::Move(Direction::Up)),
            KeyCode::Char('l') => Some(Action::Move(Direction::Right)),
            KeyCode::Left => Some(Action::MoveCursor(Direction::Left)),
            KeyCode::Right => Some(Action::MoveCursor(Direction::Right)),
            KeyCode::Up => Some(Action::MoveCursor(Direction::Up)),
            KeyCode::Down => Some(Action::MoveCursor(Direction::Down)),
            KeyCode::Char('0') => Some(Action::MoveCursorToStart),
            KeyCode::Char('$') => Some(Action::MoveCursorToEnd),
            KeyCode::Char(' ') => Some(Action::Leader),
            KeyCode::Char('i') => Some(Action::EnterInsert),
            _ => None,
        },
        AppMode::Insert => match key.code {
            KeyCode::Esc => Some(Action::ExitInsert),
            KeyCode::Left => Some(Action::MoveCursor(Direction::Left)),
            KeyCode::Right => Some(Action::MoveCursor(Direction::Right)),
            KeyCode::Up => Some(Action::MoveCursor(Direction::Up)),
            KeyCode::Down => Some(Action::MoveCursor(Direction::Down)),
            KeyCode::Home => Some(Action::MoveCursorToStart),
            KeyCode::End => Some(Action::MoveCursorToEnd),
            KeyCode::Tab => Some(Action::NextField),
            KeyCode::Enter => Some(Action::InsertNewline),
            KeyCode::Backspace => Some(Action::Backspace),
            KeyCode::Delete => Some(Action::Delete),
            KeyCode::Char('j') if panel == Panel::Method => {
                Some(Action::MoveCursor(Direction::Down))
            }
            KeyCode::Char('k') if panel == Panel::Method => Some(Action::MoveCursor(Direction::Up)),
            KeyCode::Char(c) => Some(Action::InsertChar(c)),
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::map_key;
    use crate::{
        action::{Action, Direction},
        app::{AppMode, Panel},
    };
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn j_and_k_navigate_the_method_list() {
        let down = map_key(
            key(KeyCode::Char('j')),
            AppMode::Insert,
            Panel::Method,
            false,
        );
        let up = map_key(
            key(KeyCode::Char('k')),
            AppMode::Insert,
            Panel::Method,
            false,
        );

        assert!(matches!(down, Some(Action::MoveCursor(Direction::Down))));
        assert!(matches!(up, Some(Action::MoveCursor(Direction::Up))));
    }

    #[test]
    fn leader_e_opens_history() {
        let leader = map_key(key(KeyCode::Char(' ')), AppMode::Normal, Panel::Url, false);
        let history = map_key(key(KeyCode::Char('e')), AppMode::Normal, Panel::Url, true);

        assert!(matches!(leader, Some(Action::Leader)));
        assert!(matches!(history, Some(Action::ToggleHistory)));
    }
}
