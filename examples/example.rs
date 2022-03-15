use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_schedule_states::{driver, NextState, ScheduleStates};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_states)
        .add_system(
            driver::<States>
                .exclusive_system()
                .with_run_criteria(FixedTimestep::step(1.0)),
        )
        .run();
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum States {
    StateA,
    StateB,
}

fn setup_states(mut commands: Commands) {
    let mut states = ScheduleStates::new(States::StateA);

    states
        .with_state_enter(States::StateA)
        .add_system(enter_state_a);
    states
        .with_state_update(States::StateA)
        .add_system(update_state_a)
        .add_system(change_state_a_to_b);
    states
        .with_state_exit(States::StateA)
        .add_system(exit_state_a);

    states
        .with_state_enter(States::StateB)
        .add_system(enter_state_b);
    states
        .with_state_update(States::StateB)
        .add_system(update_state_b)
        .add_system(change_state_b_to_a);
    states
        .with_state_exit(States::StateB)
        .add_system(exit_state_b);

    commands.insert_resource(NextState::<States>(None));
    commands.insert_resource(states);
}

fn update_state_a() {
    println!("update state a");
}

fn enter_state_a() {
    println!("enter state a");
}

// change state every third run of this system
// this shows that exit/enter/update all run on the same tick
fn change_state_a_to_b(mut next_state: ResMut<NextState<States>>, mut count: Local<u32>) {
    *count += 1;
    if *count > 2 {
        next_state.set(States::StateB);
        *count = 0;
    }
}

fn exit_state_a() {
    println!("exit state a");
}

fn update_state_b() {
    println!("update state b");
}

fn enter_state_b() {
    println!("enter state b");
}

fn change_state_b_to_a(mut next_state: ResMut<NextState<States>>, mut count: Local<u32>) {
    *count += 1;
    if *count > 2 {
        next_state.set(States::StateA);
        *count = 0;
    }
}

fn exit_state_b() {
    println!("exit state b");
}
