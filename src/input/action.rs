//! Key actions, the painting brush, and the reset message.

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    TogglePause,
    Reset,
}

/// A key plus modifier snapshot used as the binding lookup key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chord {
    pub key:  KeyCode,
    pub ctrl: bool,
}

impl Chord {
    pub const fn new(key: KeyCode) -> Self {
        Self { key, ctrl: false }
    }

    pub const fn ctrl(key: KeyCode) -> Self {
        Self { key, ctrl: true }
    }

    pub fn from_input(input: &KeyboardInput, ctrl: bool) -> Self {
        Self {
            key: input.key_code,
            ctrl,
        }
    }
}

/// Pointer interaction state; carries the last painted cell so drags
/// stroke a continuous line.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Brush {
    #[default]
    Idle,
    Painting {
        last: UVec2,
    },
    Erasing {
        last: UVec2,
    },
}

/// Emitted when the user requests a board reset (R key). A buffered
/// message, read by the root plugin's reset handler.
#[derive(Message, Clone, Copy)]
pub struct ResetBoard;

#[cfg(test)]
mod tests {
    use bevy::ecs::message::Messages;

    use super::*;

    #[test]
    fn default_brush_is_idle() {
        assert_eq!(Brush::default(), Brush::Idle);
    }

    #[test]
    fn reset_board_is_a_buffered_message() {
        let mut app = App::new();
        app.add_message::<ResetBoard>();
        app.world_mut().write_message(ResetBoard);
        let count = app
            .world()
            .resource::<Messages<ResetBoard>>()
            .iter_current_update_messages()
            .count();
        assert_eq!(count, 1);
    }
}
