/// This example shows that the builders are compatible with plugins

use bevy::prelude::{App, Plugin};
use bevy_prototype_schedule_states::{AppStateHelpers, StatePlugin};

fn main() {
    App::new()
        .add_plugin(StatePlugin::new(States::StateA))
        .add_plugin(StateAPlugin)
        .add_plugin(StateBPlugin);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum States {
    StateA,
    StateB,
}

struct StateAPlugin;
impl Plugin for StateAPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_state_enter(States::StateA, || println!("update state a"));
    }
}

struct StateBPlugin;
impl Plugin for StateBPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_state_update(States::StateB, || println!("update state b"));
    }
}
