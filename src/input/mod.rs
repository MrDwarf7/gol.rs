//! Input domain: key and pointer bindings, brush state, action handling.

mod action;
mod bindings;
mod handle_key;
mod handle_mouse;

use bevy::prelude::*;

use crate::SimState;
pub use crate::input::action::{Action, Brush, Chord, ResetBoard};
pub use crate::input::bindings::{Bindings, PointerBindings};
pub use crate::input::handle_key::handle_key_actions;
pub use crate::input::handle_mouse::handle_pointer_actions;

/// System sets grouping the two input pipelines so downstream domains
/// can order against them individually.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputSet {
    /// Keyboard chords -> actions (quit / pause / reset).
    Keyboard,
    /// Mouse clicks and drags -> board strokes.
    Pointer,
}

/// Converts raw keyboard/pointer input into actions on the game.
/// Systems run in `PreUpdate` so gameplay sees a consistent board.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Bindings>()
            .init_resource::<PointerBindings>()
            .init_resource::<Brush>()
            .add_message::<ResetBoard>()
            .add_systems(
                PreUpdate,
                (handle_key_actions.in_set(InputSet::Keyboard), handle_pointer_actions.in_set(InputSet::Pointer))
                    .chain(),
            );
    }
}

// Silence unused import when only referenced in doc comment contexts.
#[allow(unused_imports)]
use crate::gameplay::seed as _seed_reexport;

// Keep SimState referenced for the state machine wiring docs above.
#[allow(dead_code)]
fn _sim_state_is_used(_s: SimState) {}
