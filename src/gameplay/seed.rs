//! Pattern definitions and board seeding / reset.

use bevy::prelude::*;

use crate::gameplay::cell::CellState;
use crate::gameplay::grid::{Grid, GridError};

pub const GLIDER: &[UVec2] = &[
    UVec2::new(1, 0),
    UVec2::new(2, 1),
    UVec2::new(0, 2),
    UVec2::new(1, 2),
    UVec2::new(2, 2),
];
pub const BLINKER: &[UVec2] = &[UVec2::new(1, 0), UVec2::new(1, 1), UVec2::new(1, 2)];
pub const TOAD: &[UVec2] = &[
    UVec2::new(1, 1),
    UVec2::new(2, 1),
    UVec2::new(3, 1),
    UVec2::new(0, 2),
    UVec2::new(1, 2),
    UVec2::new(2, 2),
];
pub const BEACON: &[UVec2] = &[
    UVec2::new(0, 0),
    UVec2::new(1, 0),
    UVec2::new(0, 1),
    UVec2::new(3, 2),
    UVec2::new(2, 3),
    UVec2::new(3, 3),
];
pub const GOSPER_GUN: &[UVec2] = &[
    UVec2::new(24, 0),
    UVec2::new(22, 1),
    UVec2::new(24, 1),
    UVec2::new(12, 2),
    UVec2::new(13, 2),
    UVec2::new(20, 2),
    UVec2::new(21, 2),
    UVec2::new(34, 2),
    UVec2::new(35, 2),
    UVec2::new(11, 3),
    UVec2::new(15, 3),
    UVec2::new(20, 3),
    UVec2::new(21, 3),
    UVec2::new(34, 3),
    UVec2::new(35, 3),
    UVec2::new(0, 4),
    UVec2::new(1, 4),
    UVec2::new(10, 4),
    UVec2::new(16, 4),
    UVec2::new(20, 4),
    UVec2::new(21, 4),
    UVec2::new(0, 5),
    UVec2::new(1, 5),
    UVec2::new(10, 5),
    UVec2::new(14, 5),
    UVec2::new(16, 5),
    UVec2::new(17, 5),
    UVec2::new(22, 5),
    UVec2::new(24, 5),
    UVec2::new(10, 6),
    UVec2::new(16, 6),
    UVec2::new(24, 6),
    UVec2::new(11, 7),
    UVec2::new(15, 7),
    UVec2::new(12, 8),
    UVec2::new(13, 8),
];

/// Stamp `pattern` with its top-left corner at `origin`.
///
/// # Errors
/// Returns [`crate::gameplay::grid::GridError`] when any stamped cell lands out of
/// bounds.
pub fn stamp(grid: &mut Grid, origin: UVec2, pattern: &[UVec2]) -> Result<(), GridError> {
    pattern
        .iter()
        .try_for_each(|pos| grid.set_cell(origin + *pos, CellState::Alive))
}

fn pattern_size(pattern: &[UVec2]) -> UVec2 {
    pattern.iter().fold(UVec2::ZERO, |acc, pos| acc.max(*pos)) + UVec2::ONE
}

fn centered_origin(grid: UVec2, pattern: &[UVec2]) -> UVec2 {
    grid.saturating_sub(pattern_size(pattern)) / 2
}

/// Clear the board and lay down the classic starter scene.
pub fn reset(grid: &mut Grid) {
    grid.clear();
    let origin = centered_origin(grid.size, GOSPER_GUN);
    stamp(grid, origin, GOSPER_GUN).expect("gosper gun fits the default grid");
    let _ = stamp(grid, origin.saturating_add(UVec2::new(40, 20)), GLIDER);
    let _ = stamp(grid, origin.saturating_add(UVec2::new(8, 24)), BLINKER);
    let _ = stamp(grid, origin.saturating_add(UVec2::new(24, 28)), TOAD);
    let _ = stamp(grid, origin.saturating_add(UVec2::new(48, 8)), BEACON);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_clears_drawn_cells_then_reseeds() {
        let mut grid = Grid::new(UVec2::new(80, 60)).expect("valid");
        grid.set_cell(UVec2::ZERO, CellState::painted()).expect("in bounds");
        reset(&mut grid);
        assert_eq!(grid.cell_at(UVec2::ZERO).expect("in bounds"), CellState::Dead);
        assert!(grid.cells.contains(&CellState::Alive));
    }

    #[test]
    fn seed_classic_centers_life() {
        let mut grid = Grid::new(UVec2::new(80, 60)).expect("valid");
        reset(&mut grid);
        let min_x = grid
            .positions()
            .filter(|pos| grid.cells[grid.test_index(*pos)] == CellState::Alive)
            .map(|pos| pos.x)
            .min()
            .expect("seeded");
        assert!(min_x > 10, "life should not start on the left edge, min_x={min_x}");
    }

    #[test]
    fn stamp_rejects_out_of_bounds() {
        let mut grid = Grid::new(UVec2::splat(4)).expect("valid");
        assert!(stamp(&mut grid, UVec2::splat(3), GOSPER_GUN).is_err());
    }
}
