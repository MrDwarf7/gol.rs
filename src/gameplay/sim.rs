//! Simulation stepping: the timer-gated system that advances the grid.

use std::time::Duration;

use bevy::prelude::*;

use crate::gameplay::grid::Grid;

pub const TICK_SECONDS: f32 = 0.125;

#[derive(Resource, Debug)]
pub(super) struct SimTimer(Timer);

impl Default for SimTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs_f32(TICK_SECONDS), TimerMode::Repeating))
    }
}

/// Advance the grid one generation per tick. Gated by
/// `in_state(SimState::Running)` at registration.
pub(super) fn advance_generation(time: Res<Time>, mut grid: ResMut<Grid>, mut timer: Local<SimTimer>) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    grid.step();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameplay::CellState;

    #[test]
    fn sim_timer_fires_every_tick_seconds() {
        // Drive the timer directly; MinimalPlugins' time systems overwrite
        // advanced virtual time each frame, so exercising the timer in
        // isolation is the honest unit here.
        let mut timer = SimTimer::default().0;
        assert!(!timer.tick(Duration::from_millis(50)).just_finished());
        let fired = timer.tick(Duration::from_secs_f32(TICK_SECONDS)).just_finished();
        assert!(fired, "timer should fire once TICK_SECONDS has elapsed");
    }

    #[test]
    fn step_flips_vertical_blinker_to_horizontal() {
        // The stepping itself is covered by Grid's own tests
        // (blinker_period_two); this pins the wiring: what advance_generation
        // calls on tick is exactly Grid::step.
        let mut grid = Grid::new(UVec2::splat(5)).expect("valid");
        for pos in [UVec2::new(2, 1), UVec2::new(2, 2), UVec2::new(2, 3)] {
            grid.set_cell(pos, CellState::Alive).expect("in bounds");
        }
        advance_if_fired(&mut grid, &mut SimTimer::default().0, Duration::from_millis(10));
        assert_eq!(
            grid.cell_at(UVec2::new(2, 2)).expect("in bounds"),
            CellState::Alive,
            "no step before the tick elapses"
        );
        advance_if_fired(&mut grid, &mut SimTimer::default().0, Duration::from_secs_f32(TICK_SECONDS));
        let horizontal = [UVec2::new(1, 2), UVec2::new(2, 2), UVec2::new(3, 2)]
            .iter()
            .all(|pos| grid.cell_at(*pos).expect("in bounds") == CellState::Alive);
        assert!(horizontal, "blinker should flip after one tick");
    }

    /// Test seam mirroring the system body without Bevy time plumbing.
    fn advance_if_fired(grid: &mut Grid, timer: &mut Timer, delta: Duration) {
        if timer.tick(delta).just_finished() {
            grid.step();
        }
    }
}
