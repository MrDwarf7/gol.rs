//! Render domain: camera setup, viewport focus, and the cell sprites
//! that mirror the gameplay grid.

mod board_view;
mod camera;
pub mod palette;

use bevy::prelude::*;

use crate::render::board_view::paint_cells;
pub use crate::render::board_view::{CellPos, cell, spawn_cells};
use crate::render::camera::{ViewportFocus, spawn_camera};
pub use crate::render::palette::cell_color;

/// Owns everything the player sees: camera, board sprites, repaints.
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ViewportFocus>()
            .add_systems(Startup, (spawn_camera, board_view::spawn_cells))
            .add_systems(Update, camera::follow_viewport)
            .add_systems(PostUpdate, paint_cells);
    }
}
