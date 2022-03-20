mod app_helpers;
mod state_schedule;
use std::hash::Hash;

pub use app_helpers::AppStateHelpers;
use bevy::prelude::{App, Plugin};
pub use state_schedule::{driver, NextState, ScheduleStates};

/// This adds the `NextState` and `ScheduleStates` resources to Bevy.
/// Drivers for the states should be configured separately. The driver
/// is configured separately since when the state changes and is run
/// should be user configurable.
pub struct StatePlugin<S> {
    initial_state: S,
}

impl<S> StatePlugin<S> {
    pub fn new(initial_state: S) -> Self {
        Self { initial_state }
    }
}

impl<S> Plugin for StatePlugin<S>
where
    S: Send + Sync + 'static + Eq + Hash + Copy + Clone,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(NextState::<S>::default())
            .insert_resource(ScheduleStates::<S>::new(self.initial_state));
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::*;

    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    enum States {
        StateA,
        StateB,
    }

    #[derive(Default, Eq, PartialEq, Debug)]
    struct Counts {
        pub enters: u32,
        pub updates: u32,
        pub exits: u32,
    }

    struct StateACounts(pub Counts);
    struct StateBCounts(pub Counts);

    #[test]
    fn simple_states() {
        let mut world = World::new();

        let mut states = ScheduleStates::new(States::StateA);
        states
            .with_state_enter(States::StateA)
            .add_system(|mut counts: ResMut<StateACounts>| counts.0.enters += 1);
        states
            .with_state_update(States::StateA)
            .add_system(|mut counts: ResMut<StateACounts>| counts.0.updates += 1);
        states
            .with_state_exit(States::StateA)
            .add_system(|mut counts: ResMut<StateACounts>| counts.0.exits += 1);
        // configure systems to count state B
        states
            .with_state_enter(States::StateB)
            .add_system(|mut counts: ResMut<StateBCounts>| counts.0.enters += 1);
        states
            .with_state_update(States::StateB)
            .add_system(|mut counts: ResMut<StateBCounts>| counts.0.updates += 1);
        states
            .with_state_exit(States::StateB)
            .add_system(|mut counts: ResMut<StateBCounts>| counts.0.exits += 1);

        world.insert_resource(states);
        world.insert_resource(NextState::<States>::default());
        world.insert_resource(StateACounts(Counts::default()));
        world.insert_resource(StateBCounts(Counts::default()));

        let mut stage = SystemStage::parallel().with_system(driver::<States>.exclusive_system());
        stage.run(&mut world);

        assert_eq!(
            world.get_resource::<StateACounts>().unwrap().0,
            Counts {
                enters: 1,
                updates: 1,
                exits: 0
            }
        );

        world
            .get_resource_mut::<ScheduleStates<States>>()
            .unwrap()
            .with_state_update(States::StateA)
            .add_system(|mut next_state: ResMut<NextState<States>>| next_state.set(States::StateB));

        stage.run(&mut world);

        assert_eq!(
            world.get_resource::<StateACounts>().unwrap().0,
            Counts {
                enters: 1,
                updates: 2,
                exits: 1
            }
        );
        assert_eq!(
            world.get_resource::<StateBCounts>().unwrap().0,
            Counts {
                enters: 1,
                updates: 1,
                exits: 0
            }
        );
    }
}
