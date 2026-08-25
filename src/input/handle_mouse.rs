//! Pointer handling: translates mouse clicks and drags into board
//! strokes via the brush.

use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::{CursorMoved, PrimaryWindow};

use crate::gameplay::Grid;
use crate::input::action::Brush;
use crate::input::bindings::{PointerAction, PointerBindings};

#[allow(clippy::needless_pass_by_value)] // required by the Bevy system-param interface
pub fn handle_pointer_actions(
    mut clicks: MessageReader<MouseButtonInput>,
    mut moves: MessageReader<CursorMoved>,
    bindings: Res<PointerBindings>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut grid: ResMut<Grid>,
    mut brush: ResMut<Brush>,
) {
    let Ok(window) = window.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };

    clicks.read().for_each(|click| {
        let Some(action) = bindings.action(click.button) else {
            return;
        };
        match click.state {
            ButtonState::Pressed => {
                if let Some(cursor) = window.cursor_position() {
                    apply_at(&mut grid, &mut brush, action, cursor, camera, camera_transform);
                }
                moves.clear();
            }
            ButtonState::Released => {
                *brush = release(*brush, action);
            }
        }
    });

    match *brush {
        Brush::Painting { .. } => {
            if let Some(moved) = moves.read().last() {
                apply_at(&mut grid, &mut brush, PointerAction::PaintAlive, moved.position, camera, camera_transform);
            }
        }
        Brush::Erasing { .. } => {
            if let Some(moved) = moves.read().last() {
                apply_at(&mut grid, &mut brush, PointerAction::Erase, moved.position, camera, camera_transform);
            }
        }
        Brush::Idle => {
            moves.clear();
        }
    }
}

fn release(brush: Brush, action: PointerAction) -> Brush {
    match (brush, action) {
        // Painting ends on PaintAlive, erasing on Erase; anything else
        // keeps the current brush.
        (Brush::Painting { .. } | Brush::Erasing { .. }, _)
            if matches!(
                (&brush, &action),
                (Brush::Painting { .. }, PointerAction::PaintAlive) | (Brush::Erasing { .. }, PointerAction::Erase)
            ) =>
        {
            Brush::Idle
        }
        (brush, _) => brush,
    }
}

fn apply_at(
    grid: &mut Grid,
    brush: &mut Brush,
    action: PointerAction,
    cursor: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) {
    let Some(pos) = cell_at(cursor, camera, camera_transform, grid) else {
        return;
    };
    let state = action.state();
    match (*brush, action) {
        (Brush::Painting { last }, PointerAction::PaintAlive) | (Brush::Erasing { last }, PointerAction::Erase) => {
            grid.stroke(last, pos, state);
        }
        _ => {
            grid.set_cell(pos, state).expect("cursor cell is in bounds");
        }
    }
    *brush = match action {
        PointerAction::PaintAlive => Brush::Painting { last: pos },
        PointerAction::Erase => Brush::Erasing { last: pos },
    };
}

fn cell_at(cursor: Vec2, camera: &Camera, camera_transform: &GlobalTransform, grid: &Grid) -> Option<UVec2> {
    camera
        .viewport_to_world_2d(camera_transform, cursor)
        .ok()
        .and_then(|world| grid.world_to_cell(world))
}
