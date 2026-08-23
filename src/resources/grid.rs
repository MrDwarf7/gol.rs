use bevy::prelude::*;

use crate::assets::CELL_SIZE;
use crate::components::CellState;
use crate::error::{Error, Result};

pub const GRID_WIDTH: u32 = 80;
pub const GRID_HEIGHT: u32 = 60;

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub size:  UVec2,
    pub cells: Vec<CellState>,
}

impl Grid {
    pub fn new(size: UVec2) -> Result<Self> {
        if size.x == 0 || size.y == 0 {
            return Err(Error::InvalidDimensions(size));
        }
        let len = (size.x as usize)
            .checked_mul(size.y as usize)
            .ok_or(Error::InvalidDimensions(size))?;
        Ok(Self {
            size,
            cells: vec![CellState::Dead; len],
        })
    }

    pub fn size_from_pixels(pixels: Vec2) -> UVec2 {
        (pixels / CELL_SIZE).as_uvec2().max(UVec2::ONE)
    }

    pub fn resize(&mut self, size: UVec2) -> Result<()> {
        if size == self.size {
            return Ok(());
        }
        let mut next = Self::new(size)?;
        self.positions().for_each(|pos| {
            if pos.x < size.x && pos.y < size.y {
                let idx = next.index(pos);
                next.cells[idx] = self.cells[self.index(pos)];
            }
        });
        *self = next;
        Ok(())
    }

    pub fn positions(&self) -> impl Iterator<Item = UVec2> + '_ {
        (0..self.size.y).flat_map(move |y| (0..self.size.x).map(move |x| UVec2::new(x, y)))
    }

    pub fn cell_at(&self, pos: UVec2) -> Result<CellState> {
        let idx = self.checked_index(pos)?;
        Ok(self.cells[idx])
    }

    pub fn set_cell(&mut self, pos: UVec2, state: CellState) -> Result<()> {
        let idx = self.checked_index(pos)?;
        self.cells[idx] = state;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.cells.fill(CellState::Dead);
    }

    pub fn stroke(&mut self, from: UVec2, to: UVec2, state: CellState) {
        cells_on_line(from, to).into_iter().for_each(|pos| {
            self.set_cell(pos, state)
                .expect("line between in-bounds cells stays in bounds");
        });
    }

    pub fn world_size(&self) -> Vec2 {
        self.size.as_vec2() * CELL_SIZE
    }

    pub fn world_to_cell(&self, world: Vec2) -> Option<UVec2> {
        let area = self.world_size();
        let from_tl = Vec2::new(world.x + area.x / 2.0, area.y / 2.0 - world.y);
        if from_tl.x < 0.0 || from_tl.y < 0.0 {
            return None;
        }
        let pos = (from_tl / CELL_SIZE).as_uvec2();
        (pos.x < self.size.x && pos.y < self.size.y).then_some(pos)
    }

    pub(crate) fn index(&self, pos: UVec2) -> usize {
        debug_assert!(pos.x < self.size.x && pos.y < self.size.y);
        (pos.y as usize) * (self.size.x as usize) + (pos.x as usize)
    }

    pub(crate) fn wrap(&self, pos: UVec2, delta: IVec2) -> UVec2 {
        UVec2::new(
            (pos.x as i32 + delta.x).rem_euclid(self.size.x as i32) as u32,
            (pos.y as i32 + delta.y).rem_euclid(self.size.y as i32) as u32,
        )
    }

    pub(crate) fn live_count(&self, pos: UVec2) -> u8 {
        const OFFSETS: [IVec2; 8] = [
            IVec2::new(-1, -1),
            IVec2::new(0, -1),
            IVec2::new(1, -1),
            IVec2::new(-1, 0),
            IVec2::new(1, 0),
            IVec2::new(-1, 1),
            IVec2::new(0, 1),
            IVec2::new(1, 1),
        ];
        OFFSETS
            .iter()
            .filter(|delta| self.cells[self.index(self.wrap(pos, **delta))].is_live())
            .count() as u8
    }

    fn checked_index(&self, pos: UVec2) -> Result<usize> {
        if pos.x >= self.size.x || pos.y >= self.size.y {
            return Err(Error::OutOfBounds { pos, size: self.size });
        }
        Ok(self.index(pos))
    }
}

fn cells_on_line(from: UVec2, to: UVec2) -> Vec<UVec2> {
    let mut x0 = from.x as i32;
    let mut y0 = from.y as i32;
    let x1 = to.x as i32;
    let y1 = to.y as i32;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut cells = Vec::new();
    loop {
        cells.push(UVec2::new(x0 as u32, y0 as u32));
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    cells
}

impl TryFrom<UVec2> for Grid {
    type Error = Error;

    fn try_from(size: UVec2) -> Result<Self> {
        Self::new(size)
    }
}

impl TryFrom<Vec2> for Grid {
    type Error = Error;

    fn try_from(pixels: Vec2) -> Result<Self> {
        Self::new(Self::size_from_pixels(pixels))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_zero_width() {
        let err = Grid::new(UVec2::new(0, 10)).expect_err("zero width");
        assert!(matches!(err, Error::InvalidDimensions(size) if size == UVec2::new(0, 10)));
    }

    #[test]
    fn new_rejects_zero_height() {
        let err = Grid::new(UVec2::new(10, 0)).expect_err("zero height");
        assert!(matches!(err, Error::InvalidDimensions(size) if size == UVec2::new(10, 0)));
    }

    #[test]
    fn try_from_uvec2_matches_new() {
        let size = UVec2::new(3, 2);
        assert_eq!(Grid::try_from(size).expect("valid"), Grid::new(size).expect("valid"));
    }

    #[test]
    fn new_grid_is_all_dead() {
        let grid = Grid::new(UVec2::new(3, 2)).expect("valid");
        assert_eq!(grid.cells.len(), 6);
        assert!(grid.cells.iter().all(|c| *c == CellState::Dead));
    }

    #[test]
    fn cell_at_rejects_out_of_bounds() {
        let grid = Grid::new(UVec2::splat(2)).expect("valid");
        let err = grid.cell_at(UVec2::new(2, 0)).expect_err("x out of bounds");
        assert!(matches!(
            err,
            Error::OutOfBounds { pos, size } if pos == UVec2::new(2, 0) && size == UVec2::splat(2)
        ));
    }

    #[test]
    fn world_to_cell_maps_origin_to_center() {
        let grid = Grid::new(UVec2::splat(4)).expect("valid");
        assert_eq!(grid.world_to_cell(Vec2::ZERO), Some(UVec2::splat(2)));
    }

    #[test]
    fn world_to_cell_rejects_outside() {
        let grid = Grid::new(UVec2::splat(4)).expect("valid");
        assert_eq!(grid.world_to_cell(Vec2::new(-100.0, 0.0)), None);
    }

    #[test]
    fn clear_kills_every_cell() {
        let mut grid = Grid::new(UVec2::splat(3)).expect("valid");
        grid.set_cell(UVec2::ZERO, CellState::Alive).expect("in bounds");
        grid.set_cell(UVec2::splat(1), CellState::painted()).expect("in bounds");
        grid.clear();
        assert!(grid.cells.iter().all(|c| *c == CellState::Dead));
    }

    #[test]
    fn live_count_includes_shielded() {
        let mut grid = Grid::new(UVec2::splat(3)).expect("valid");
        grid.set_cell(UVec2::ZERO, CellState::painted()).expect("in bounds");
        assert_eq!(grid.live_count(UVec2::new(1, 1)), 1);
    }

    #[test]
    fn stroke_fills_horizontal_line() {
        let mut grid = Grid::new(UVec2::splat(5)).expect("valid");
        grid.stroke(UVec2::new(0, 2), UVec2::new(4, 2), CellState::painted());
        (0..5).for_each(|x| {
            assert_eq!(grid.cell_at(UVec2::new(x, 2)).expect("in bounds"), CellState::painted());
        });
        assert_eq!(grid.cell_at(UVec2::ZERO).expect("in bounds"), CellState::Dead);
    }

    #[test]
    fn stroke_same_cell_paints_once() {
        let mut grid = Grid::new(UVec2::splat(3)).expect("valid");
        grid.stroke(UVec2::splat(1), UVec2::splat(1), CellState::painted());
        assert_eq!(grid.cell_at(UVec2::splat(1)).expect("in bounds"), CellState::painted());
        assert_eq!(grid.cells.iter().filter(|c| c.is_live()).count(), 1);
    }

    #[test]
    fn set_cell_round_trips() {
        let mut grid = Grid::new(UVec2::splat(2)).expect("valid");
        grid.set_cell(UVec2::splat(1), CellState::Alive).expect("in bounds");
        assert_eq!(grid.cell_at(UVec2::splat(1)).expect("in bounds"), CellState::Alive);
        assert_eq!(grid.cell_at(UVec2::ZERO).expect("in bounds"), CellState::Dead);
    }

    #[test]
    fn size_from_pixels_divides_by_cell_size() {
        assert_eq!(Grid::size_from_pixels(Vec2::new(800.0, 600.0)), UVec2::new(80, 60));
    }

    #[test]
    fn try_from_window_pixels_builds_grid() {
        let grid = Grid::try_from(Vec2::new(800.0, 600.0)).expect("valid");
        assert_eq!(grid.size, UVec2::new(80, 60));
    }

    #[test]
    fn resize_keeps_overlap_and_grows_dead() {
        let mut grid = Grid::new(UVec2::splat(2)).expect("valid");
        grid.set_cell(UVec2::ZERO, CellState::Alive).expect("in bounds");
        grid.set_cell(UVec2::splat(1), CellState::Alive).expect("in bounds");
        grid.resize(UVec2::splat(3)).expect("valid");
        assert_eq!(grid.size, UVec2::splat(3));
        assert_eq!(grid.cell_at(UVec2::ZERO).expect("in bounds"), CellState::Alive);
        assert_eq!(grid.cell_at(UVec2::splat(1)).expect("in bounds"), CellState::Alive);
        assert_eq!(grid.cell_at(UVec2::splat(2)).expect("in bounds"), CellState::Dead);
    }

    #[test]
    fn resize_shrink_drops_cells_outside() {
        let mut grid = Grid::new(UVec2::splat(3)).expect("valid");
        grid.set_cell(UVec2::splat(2), CellState::Alive).expect("in bounds");
        grid.resize(UVec2::splat(2)).expect("valid");
        assert_eq!(grid.size, UVec2::splat(2));
        assert_eq!(grid.cells.len(), 4);
    }
}
