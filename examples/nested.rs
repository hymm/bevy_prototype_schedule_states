/// toggle between `StartMenu` state and `Playing` state with space bar
/// while in the `Playing` state toggle between running and paused with the escape key
/// The bevy window will need focus to detect the keystrokes, but output is to the console.
use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_schedule_states::{
    driver, AppStateHelpers, NextState, ScheduleStates, StatePlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // configure GameState
        .add_plugin(StatePlugin::new(GameState::StartMenu))
        .add_system_to_state_update(GameState::StartMenu, || println!("start"))
        .add_system_to_state_update(GameState::Playing, || println!("playing"))
        .add_system(
            driver::<GameState>
                .exclusive_system()
                .with_run_criteria(FixedTimestep::step(1.0)),
        )
        // configure PlayingState
        .add_plugin(StatePlugin::new(PlayingState::Running))
        .add_system_to_state_update(PlayingState::Running, || println!("running"))
        .add_system_to_state_update(PlayingState::Paused, || println!("paused"))
        // `add_nested_driver_to_state` adds the driver to the update state
        // and adds systems that fire on enter and on exit for the current `PlayingState`
        // when `GameState::Playing` is entered or exited respectively
        .add_nested_driver_to_state::<GameState, PlayingState>(GameState::Playing)
        // add some input detection systems to toggle state
        .add_system(toggle_playing)
        .add_system(toggle_paused)
        .run();
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum GameState {
    StartMenu,
    Playing,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum PlayingState {
    Running,
    Paused,
}

fn toggle_playing(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    current_state: Res<ScheduleStates<GameState>>,
) {
    if input.just_pressed(KeyCode::Space) {
        input.clear_just_pressed(KeyCode::Space);
        match current_state.current_state() {
            GameState::StartMenu => game_state.set(GameState::Playing),
            GameState::Playing => game_state.set(GameState::StartMenu),
        }
    }
}

fn toggle_paused(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<NextState<PlayingState>>,
    current_state: Res<ScheduleStates<PlayingState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        input.clear_just_pressed(KeyCode::Escape);
        match current_state.current_state() {
            PlayingState::Running => game_state.set(PlayingState::Paused),
            PlayingState::Paused => game_state.set(PlayingState::Running),
        }
    }
}
