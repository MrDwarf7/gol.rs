//! Cell states and the B3/S23 survival rule.

/// Ticks a freshly painted [`CellState::Shielded`] cell persists before
/// it starts following the normal rules again.
pub const DRAW_SHIELD_TICKS: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CellState {
    Alive,
    Shielded(u8),
    #[default]
    Dead,
}

impl CellState {
    #[must_use]
    pub fn is_live(self) -> bool {
        !matches!(self, Self::Dead)
    }

    /// State assigned when the user paints on the board.
    #[must_use]
    pub fn painted() -> Self {
        Self::Shielded(DRAW_SHIELD_TICKS)
    }

    /// Advance one generation under B3/S23; shielded cells ignore
    /// underpopulation until their last tick, then expire.
    #[must_use]
    pub fn next(self, live_neighbors: u8) -> Self {
        match self {
            Self::Shielded(ticks) if ticks > 1 => Self::Shielded(ticks - 1),
            Self::Shielded(_) | Self::Alive => {
                match live_neighbors {
                    2 | 3 => Self::Alive,
                    _ => Self::Dead,
                }
            }
            Self::Dead if live_neighbors == 3 => Self::Alive,
            Self::Dead => Self::Dead,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_includes_alive_and_shielded() {
        assert!(CellState::Alive.is_live());
        assert!(CellState::painted().is_live());
        assert!(!CellState::Dead.is_live());
    }

    #[test]
    fn dead_becomes_alive_with_three_neighbors() {
        assert_eq!(CellState::Dead.next(3), CellState::Alive);
    }

    #[test]
    fn alive_survives_with_two_or_three_neighbors() {
        assert_eq!(CellState::Alive.next(2), CellState::Alive);
        assert_eq!(CellState::Alive.next(3), CellState::Alive);
    }

    #[test]
    fn alive_dies_under_or_over_populated() {
        assert_eq!(CellState::Alive.next(1), CellState::Dead);
        assert_eq!(CellState::Alive.next(4), CellState::Dead);
    }

    #[test]
    fn dead_stays_dead_without_three_neighbors() {
        assert_eq!(CellState::Dead.next(2), CellState::Dead);
    }

    #[test]
    fn shielded_ignores_underpopulation() {
        assert_eq!(CellState::Shielded(4).next(0), CellState::Shielded(3));
    }

    #[test]
    fn shield_expiry_applies_b3s23() {
        assert_eq!(CellState::Shielded(1).next(0), CellState::Dead);
        assert_eq!(CellState::Shielded(1).next(2), CellState::Alive);
    }
}
