//! Conway's Game of Life as a Bevy library.
//!
//! The crate is split by domain, mirroring the `bevy_game_template`:
//! - [`gameplay`]: grid resource, rules, stepping, seeding
//! - [`input`]: key/pointer bindings and brush handling
//! - [`render`]: camera, viewport focus, board sprites
//!
//! [`SimState`] is a real Bevy state (`States` derive). Pause/resume is
//! a `NextState<SimState>` write from input; gameplay systems gate on
//! `in_state(SimState::Running)`.
//!
//! Startup order: `PreStartup` builds the fixed [`Grid`] from the
//! initial window; `Startup` seeds the classic scene then spawns one
//! sprite per cell; `Update` steps the simulation on a 0.125 s tick
//! while Running; `PostUpdate` repaints sprites unconditionally so a
//! paused reset still redraws the board.

pub mod error;
pub mod gameplay;
pub mod input;
pub mod render;

use bevy::prelude::*;

use crate::gameplay::{Grid, seed};
use crate::input::{Brush, ResetBoard};

/// Root plugin: owns the shared pause state and composes the domains.
pub struct GolPlugin;

impl Plugin for GolPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimState>()
            .insert_resource(ClearColor(render::palette::CLEAR))
            .add_plugins((gameplay::GameplayPlugin, input::InputPlugin, render::RenderPlugin))
            .add_message::<ResetBoard>()
            // Cross-domain glue lives at the root: reset clears the grid
            // (gameplay) and drops any in-progress stroke (input).
            .add_systems(PreUpdate, handle_reset_board)
            .add_systems(Startup, seed_classic);
    }
}

/// Whether the simulation advances or holds still for drawing.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimState {
    #[default]
    Running,
    Paused,
}

fn seed_classic(mut grid: ResMut<Grid>) {
    seed::reset(&mut grid);
}

fn handle_reset_board(mut resets: MessageReader<ResetBoard>, mut grid: ResMut<Grid>, mut brush: ResMut<Brush>) {
    if resets.read().next().is_none() {
        return;
    }
    *brush = Brush::Idle;
    seed::reset(&mut grid);
}

pub use crate::error::{Error, Result};
