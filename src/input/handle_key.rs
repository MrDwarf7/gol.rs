//! Keyboard handling: translates key chords into actions.

use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use crate::SimState;
use crate::input::action::{Action, Chord, ResetBoard};
use crate::input::bindings::Bindings;

#[allow(clippy::needless_pass_by_value)] // required by the Bevy system-param interface
pub fn handle_key_actions(
    mut inputs: MessageReader<KeyboardInput>,
    bindings: Res<Bindings>,
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<SimState>>,
    mut exit: MessageWriter<AppExit>,
    mut next_state: ResMut<NextState<SimState>>,
    mut reset: MessageWriter<ResetBoard>,
) {
    let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
    inputs
        .read()
        .filter(|input| input.state == ButtonState::Pressed && !input.repeat)
        .filter_map(|input| bindings.action(Chord::from_input(input, ctrl)))
        .for_each(|action| {
            match action {
                Action::Quit => {
                    exit.write(AppExit::Success);
                }
                Action::TogglePause => {
                    match *state.get() {
                        SimState::Running => next_state.set(SimState::Paused),
                        SimState::Paused => next_state.set(SimState::Running),
                    }
                }
                Action::Reset => {
                    reset.write(ResetBoard);
                }
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameplay::Grid;

    #[test]
    fn space_toggles_pause_via_next_state() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(bevy::state::app::StatesPlugin)
            .init_state::<SimState>()
            .add_message::<KeyboardInput>()
            .add_message::<AppExit>()
            .add_message::<ResetBoard>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<Bindings>()
            .add_systems(Update, handle_key_actions);

        // Press Space; the transition applies on the next frame's
        // StateTransition pass, so run two frames.
        press_key(&mut app, KeyCode::Space);
        app.update();
        app.update();

        assert_eq!(*app.world().resource::<State<SimState>>().get(), SimState::Paused);

        press_key(&mut app, KeyCode::Space);
        app.update();
        app.update();

        assert_eq!(*app.world().resource::<State<SimState>>().get(), SimState::Running);
    }

    #[test]
    fn r_key_writes_reset_message() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(bevy::state::app::StatesPlugin)
            .init_state::<SimState>()
            .add_message::<ResetBoard>()
            .insert_resource(Bindings::default())
            .insert_resource(Grid::new(UVec2::new(80, 60)).expect("valid"))
            .add_systems(Update, handle_key_actions);

        press_key(&mut app, KeyCode::KeyR);
        app.update();

        // The system wrote the message this frame; a reader added after
        // the update still sees it because messages buffer for two frames.
        let count = app
            .world()
            .resource::<Messages<ResetBoard>>()
            .iter_current_update_messages()
            .count();
        assert_eq!(count, 1, "reset message should be written");
    }

    fn press_key(app: &mut App, key: KeyCode) {
        let name = format!("{key:?}");
        app.init_resource::<ButtonInput<KeyCode>>();
        app.add_message::<KeyboardInput>();
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(key);
        app.world_mut().write_message(KeyboardInput {
            key_code:    key,
            state:       bevy::input::ButtonState::Pressed,
            repeat:      false,
            text:        None,
            logical_key: bevy::input::keyboard::Key::Character(name.into()),
            window:      Entity::PLACEHOLDER,
        });
    }
}
