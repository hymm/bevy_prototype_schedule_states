/// toggle between `StartMenu` state and `Playing` state with spacebar
/// while in the `Playing` state toggle between running and paused with the escape key
/// The bevy window will need focus to detect the keystrokes, but output is to the console.
use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_schedule_states::{driver, NextState, ScheduleStates, StatePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StatePlugin::new(GameState::StartMenu))
        .insert_resource(get_game_state_schedules())
        .add_plugin(StatePlugin::new(PlayingState::Running))
        .insert_resource(get_playing_state_schedules())
        .add_system(
            driver::<GameState>
                .exclusive_system()
                .with_run_criteria(FixedTimestep::step(1.0)),
        )
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

fn build_game_states() -> ScheduleStates<GameState> {
    let mut states = ScheduleStates::new(GameState::StartMenu);
    states
        .with_state_update(GameState::StartMenu)
        .add_system(while_start);
    states
        .with_state_update(GameState::Playing)
        .add_system(while_playing)
        .add_system(toggle_paused);

    states.add_nested_driver_to_state::<PlayingState>(GameState::Playing);

    states
}

fn build_playing_states() -> ScheduleStates<PlayingState> {
    let mut states = ScheduleStates::new(PlayingState::Running);
    states
        .with_state_update(PlayingState::Running)
        .add_system(while_running);
    states
        .with_state_update(PlayingState::Paused)
        .add_system(while_paused);
    states
}

fn while_playing() {
    println!("playing");
}

fn while_start() {
    println!("start");
}

fn while_paused() {
    println!("paused");
}

fn while_running() {
    println!("running");
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
        println!("blah");
        match current_state.current_state() {
            PlayingState::Running => game_state.set(PlayingState::Paused),
            PlayingState::Paused => game_state.set(PlayingState::Running),
        }
    }
}
