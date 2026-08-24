//! The camera and its viewport focus. The window is a viewport onto the
//! fixed board canvas; resizing the window does not resize the world --
//! the orthographic projection keeps the full canvas framed.

use bevy::camera::ScalingMode;
use bevy::prelude::*;

/// World-space point the camera centers on. Defaults to the grid center
/// (world origin).
#[derive(Resource, Debug, Clone, Copy, Default, Deref, DerefMut)]
pub struct ViewportFocus(pub Vec2);

pub fn spawn_camera(mut commands: Commands, grid: Res<Grid>) {
    let area = grid.world_size();
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width:  area.x,
                height: area.y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn follow_viewport(focus: Res<ViewportFocus>, mut camera: Query<&mut Transform, With<Camera2d>>) {
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };
    transform.translation.x = focus.x;
    transform.translation.y = focus.y;
}

use crate::gameplay::Grid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follow_viewport_writes_focus_to_camera() {
        let mut app = App::new();
        app.insert_resource(ViewportFocus(Vec2::new(3.0, 4.0)))
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn(Camera2d);
            })
            .add_systems(Update, follow_viewport);
        app.update();

        let transform = app
            .world_mut()
            .query::<&Transform>()
            .single(app.world())
            .expect("camera");
        assert_eq!(transform.translation.x, 3.0);
        assert_eq!(transform.translation.y, 4.0);
    }
}
