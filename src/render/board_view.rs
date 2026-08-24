//! Cell sprite entities and the repaint pass that mirrors grid state
//! into their colors.

use bevy::prelude::*;

use crate::gameplay::{CELL_SIZE, CellState, Grid};
use crate::render::palette::cell_color;

/// Grid position of a spawned cell entity, kept in sync with `Grid`.
///
/// Tuple newtype over `Vec2`; `Deref` exposes the inner vector so
/// callers can treat a `CellPos` as its position directly.
#[derive(Component, Debug, Clone, Copy, Deref)]
pub struct CellPos(pub Vec2);

impl From<Vec2> for CellPos {
    fn from(pos: Vec2) -> Self {
        Self(pos)
    }
}

impl From<UVec2> for CellPos {
    fn from(pos: UVec2) -> Self {
        Self(pos.as_vec2())
    }
}

impl From<CellPos> for Vec2 {
    fn from(cell: CellPos) -> Self {
        cell.0
    }
}

impl From<CellPos> for UVec2 {
    fn from(cell: CellPos) -> Self {
        cell.0.as_uvec2()
    }
}

/// Build the component bundle for one board cell.
pub fn cell(state: CellState, pos: Vec2, origin: Vec2) -> (CellPos, Sprite, Transform) {
    (
        CellPos::from(pos),
        Sprite::from_color(cell_color(state), Vec2::splat(CELL_SIZE - 1.0)),
        Transform::from_translation(origin.extend(0.0)),
    )
}

/// Spawn one sprite entity per grid slot. Runs at Startup, after seeding.
pub fn spawn_cells(mut commands: Commands, grid: Res<Grid>) {
    let extent = Vec2::splat(CELL_SIZE);
    let origin = Vec2::new(
        -0.5 * grid.size.x as f32 * extent.x + 0.5 * extent.x,
        0.5 * grid.size.y as f32 * extent.y - 0.5 * extent.y,
    );
    grid.positions().for_each(|pos| {
        let p = pos.as_vec2();
        let world = origin + Vec2::new(p.x * extent.x, -p.y * extent.y);
        let state = grid.cell_at(pos).expect("startup iterates in-bounds cells");
        commands.spawn(cell(state, p, world));
    });
}

/// Mirror grid state into sprite colors every frame, in PostUpdate so a
/// paused reset still repaints the board.
pub fn paint_cells(grid: Res<Grid>, mut cells: Query<(&CellPos, &mut Sprite)>) {
    cells.iter_mut().for_each(|(cell_pos, mut sprite)| {
        let state = grid
            .cell_at(UVec2::from(*cell_pos))
            .expect("cell entities match the grid");
        sprite.color = cell_color(state);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::palette;

    #[test]
    fn cell_pos_round_trips_vec2() {
        let pos = Vec2::new(3.0, 4.0);
        assert_eq!(Vec2::from(CellPos::from(pos)), pos);
    }

    #[test]
    fn cell_pos_round_trips_uvec2() {
        let pos = UVec2::new(3, 4);
        assert_eq!(UVec2::from(CellPos::from(pos)), pos);
    }

    #[test]
    fn spawn_cells_creates_one_entity_per_slot() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(Grid::new(UVec2::new(80, 60)).expect("valid"))
            .add_systems(Startup, spawn_cells);
        app.update();

        let world = app.world_mut();
        let mut query = world.query::<&CellPos>();
        let count = query.iter(world).count();
        assert_eq!(count, (80 * 60) as usize);
    }

    #[test]
    fn origin_cell_is_left_of_and_above_world_origin() {
        let grid = Grid::new(UVec2::splat(4)).expect("valid");
        let extent = Vec2::splat(CELL_SIZE);
        let origin = Vec2::new(
            -0.5 * grid.size.x as f32 * extent.x + 0.5 * extent.x,
            0.5 * grid.size.y as f32 * extent.y - 0.5 * extent.y,
        );
        assert!(origin.x < 0.0);
        assert!(origin.y > 0.0);
    }

    #[test]
    fn colors_are_distinct_per_state() {
        assert_ne!(cell_color(CellState::Dead), cell_color(CellState::Alive));
        assert_ne!(cell_color(CellState::Alive), cell_color(CellState::painted()));
        assert_ne!(palette::CLEAR, cell_color(CellState::Dead));
    }
}
