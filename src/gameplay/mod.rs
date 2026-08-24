//! Gameplay domain: board state, rules, stepping, and seeding.

mod cell;
mod grid;
pub(crate) mod seed;
mod sim;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::SimState;
pub use crate::gameplay::cell::{CellState, DRAW_SHIELD_TICKS};
pub use crate::gameplay::grid::{CELL_SIZE, Grid, GridError};

/// Owns everything simulation-side. The grid is built in `PreStartup`
/// from the initial window so the canvas is fixed for the app lifetime.
/// Stepping runs only while [`SimState::Running`]; pause/resume is a
/// Bevy `States` machine owned by the root plugin.
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_grid_from_window)
            .add_systems(Update, sim::advance_generation.run_if(in_state(SimState::Running)));
    }
}

fn init_grid_from_window(world: &mut World) {
    let pixels = {
        let mut windows = world.query_filtered::<&Window, With<PrimaryWindow>>();
        let Ok(window) = windows.single(world) else {
            return;
        };
        window.resolution.size()
    };
    let grid = Grid::try_from(pixels).expect("window fits at least one cell");
    world.insert_resource(grid);
}
