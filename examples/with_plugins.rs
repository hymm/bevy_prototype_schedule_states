/// This example shows that the builders are compatible with plugins
use bevy::{
    prelude::{App, Plugin, IntoExclusiveSystem, ExclusiveSystemDescriptorCoercion},
    DefaultPlugins, core::FixedTimestep,
};
use bevy_prototype_schedule_states::{AppStateHelpers, StatePlugin, driver};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StatePlugin::new(States::StateA))
        .add_plugin(StateAPlugin)
        .add_plugin(StateBPlugin)
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

struct StateAPlugin;
impl Plugin for StateAPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_update(States::StateA, || println!("update state a"));
    }
}

struct StateBPlugin;
impl Plugin for StateBPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_update(States::StateB, || println!("update state b"));
    }
}
