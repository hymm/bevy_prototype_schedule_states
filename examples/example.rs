use bevy::{prelude::*};
use bevy_prototype_schedule_states::ScheduleStates;

fn main() {
    App::new().add_startup_system(setup_states).run();
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum States {
  StateA,
  StateB
}

fn setup_states(mut commands: Commands) {
  let mut states = ScheduleStates::new(States::StateA);

  states.on_update_with_system(States::StateA, update_state_a);

  commands.insert_resource(states);
}

fn update_state_a() {
  println!("update state a");
}
