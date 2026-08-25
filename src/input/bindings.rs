//! Keyboard and mouse binding tables.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::gameplay::CellState;
use crate::input::action::{Action, Chord};

/// Which pointer action a mouse button maps to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointerAction {
    PaintAlive,
    Erase,
}

impl PointerAction {
    /// The cell state this action paints with.
    pub fn state(self) -> CellState {
        match self {
            Self::PaintAlive => CellState::painted(),
            Self::Erase => CellState::Dead,
        }
    }
}

#[derive(Resource, Debug)]
pub struct Bindings {
    map: HashMap<Chord, Action>,
}

impl Bindings {
    #[must_use]
    pub fn action(&self, chord: Chord) -> Option<Action> {
        self.map.get(&chord).copied()
    }
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            map: HashMap::from([
                (Chord::new(KeyCode::Escape), Action::Quit),
                (Chord::new(KeyCode::KeyQ), Action::Quit),
                (Chord::ctrl(KeyCode::KeyQ), Action::Quit),
                (Chord::new(KeyCode::Space), Action::TogglePause),
                (Chord::new(KeyCode::KeyR), Action::Reset),
            ]),
        }
    }
}

#[derive(Resource, Debug)]
pub struct PointerBindings {
    map: HashMap<MouseButton, PointerAction>,
}

impl PointerBindings {
    #[must_use]
    pub fn action(&self, button: MouseButton) -> Option<PointerAction> {
        self.map.get(&button).copied()
    }
}

impl Default for PointerBindings {
    fn default() -> Self {
        Self {
            map: HashMap::from([
                (MouseButton::Left, PointerAction::PaintAlive),
                (MouseButton::Right, PointerAction::Erase),
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bindings_map_quit_chords() {
        let bindings = Bindings::default();
        assert_eq!(bindings.action(Chord::new(KeyCode::Escape)), Some(Action::Quit));
        assert_eq!(bindings.action(Chord::new(KeyCode::KeyQ)), Some(Action::Quit));
        assert_eq!(bindings.action(Chord::ctrl(KeyCode::KeyQ)), Some(Action::Quit));
        assert_eq!(bindings.action(Chord::new(KeyCode::KeyA)), None);
        assert_eq!(bindings.action(Chord::new(KeyCode::Space)), Some(Action::TogglePause));
        assert_eq!(bindings.action(Chord::new(KeyCode::KeyR)), Some(Action::Reset));
    }

    #[test]
    fn left_click_paints_alive() {
        let bindings = PointerBindings::default();
        assert_eq!(bindings.action(MouseButton::Left), Some(PointerAction::PaintAlive));
        assert_eq!(bindings.action(MouseButton::Right), Some(PointerAction::Erase));
        assert_eq!(PointerAction::Erase.state(), CellState::Dead);
    }
}
