//! The cell color palette and window clear color.

use bevy::prelude::*;

use crate::gameplay::CellState;

pub const CLEAR: Color = Color::srgb(0.04, 0.04, 0.05);

const ALIVE: Color = Color::srgb(0.82, 0.92, 0.55);
const SHIELDED: Color = Color::srgb(0.95, 0.98, 0.75);
const DEAD: Color = Color::srgb(0.10, 0.11, 0.13);

pub fn clear_color() -> Color {
    CLEAR
}

/// Map a cell state to its display color.
pub fn cell_color(state: CellState) -> Color {
    match state {
        CellState::Alive => ALIVE,
        CellState::Shielded(_) => SHIELDED,
        CellState::Dead => DEAD,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alive_and_dead_map_to_distinct_colors() {
        assert_ne!(cell_color(CellState::Alive), cell_color(CellState::Dead));
        assert_eq!(cell_color(CellState::painted()), SHIELDED);
        assert_eq!(cell_color(CellState::Dead), DEAD);
        assert_ne!(cell_color(CellState::painted()), ALIVE);
    }
}
