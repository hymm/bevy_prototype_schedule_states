use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_schedule_states::{driver, AppStateHelpers, NextState, StatePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // adding the `StatePlugin` adds a `ScheduleStates<States>` resource and
        // a `NextState<States>` resource.
        // make sure you add the `StatePlugin` before trying to use the builder methods
        // on the state.  Failing to do so will cause a panic as the `ScheduleStates`
        // resource will not be available
        .add_plugin(StatePlugin::<States>::new(States::StateA))
        // importing the `AppStateHelpers` trait adds builder methods on app to
        // configure running systems on the states
        .add_system_to_enter(States::StateA, || println!("enter state a"))
        .add_system_set_to_update(
            States::StateA,
            SystemSet::new()
                .with_system(|| println!("update state a"))
                .with_system(change_state_a_to_b),
        )
        .add_system_to_exit(States::StateA, || println!("exit state a"))
        .add_system_to_enter(States::StateB, || println!("enter state b"))
        .add_system_set_to_update(
            States::StateB,
            SystemSet::new()
                .with_system(|| println!("update state b"))
                .with_system(change_state_b_to_a),
        )
        .add_system_to_exit(States::StateB, || println!("exit state b"))
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

// change state every third run of this system
// this shows that exit/enter/update all run on the same tick
// use `ResMut<NextState<States>>` to change the state
fn change_state_a_to_b(mut next_state: ResMut<NextState<States>>, mut count: Local<u32>) {
    *count += 1;
    if *count > 2 {
        // call set to change the state, the state will be applied the next
        // time the state `driver` is called or if the state is changed from within the state
        // like it is here the state will change once the current schedule is done running
        next_state.set(States::StateB);
        *count = 0;
    }
}

fn change_state_b_to_a(mut next_state: ResMut<NextState<States>>, mut count: Local<u32>) {
    *count += 1;
    if *count > 2 {
        next_state.set(States::StateA);
        *count = 0;
    }
}
