use bevy::{prelude::*, core::FixedTimestep};
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

    states.add_state(States::StateA);
    states.on_enter_with_system(States::StateA, enter_state_a);
    states.on_update_with_system(States::StateA, update_state_a);
    states.on_update_with_system(States::StateA, change_state_a_to_b);
    states.on_exit_with_system(States::StateA, exit_state_a);

    states.add_state(States::StateB);
    states.on_enter_with_system(States::StateB, enter_state_b);
    states.on_update_with_system(States::StateB, update_state_b);
    states.on_update_with_system(States::StateB, change_state_b_to_a);
    states.on_exit_with_system(States::StateB, exit_state_b);

    commands.insert_resource(NextState::<States>(None));
    commands.insert_resource(states);
}

fn update_state_a() {
    println!("update state a");
}

fn enter_state_a() {
    println!("enter state a");
}

fn change_state_a_to_b(mut next_state: ResMut<NextState<States>>, mut count: Local<u32>) {
  *count += 1;
  if *count > 2 {
    next_state.0 = Some(States::StateB);
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
    next_state.0 = Some(States::StateA);
    *count = 0;
  }
}

fn exit_state_b() {
    println!("exit state b");
}
