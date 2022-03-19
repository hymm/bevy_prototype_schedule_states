use bevy::{
    ecs::schedule::IntoSystemDescriptor,
    prelude::{App, SystemSet},
};
use std::hash::Hash;

use crate::ScheduleStates;

pub trait AppStateHelpers {
    fn add_system_to_state_update<S, Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;

    fn add_system_to_state_enter<S, Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;

    fn add_system_to_state_exit<S, Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;

    fn add_system_set_to_state_update<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;

    fn add_system_set_to_state_enter<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;

    fn add_system_set_to_state_exit<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;
}

impl AppStateHelpers for App {
    fn add_system_to_state_update<S, Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static,
    {
        self.world
            .get_resource_mut::<ScheduleStates<S>>()
            .unwrap()
            .with_state_update(state)
            .add_system(system);

        self
    }

    fn add_system_to_state_enter<S, Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static,
    {
        self.world
            .get_resource_mut::<ScheduleStates<S>>()
            .unwrap()
            .with_state_enter(state)
            .add_system(system);

        self
    }

    fn add_system_to_state_exit<S, Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static,
    {
        self.world
            .get_resource_mut::<ScheduleStates<S>>()
            .unwrap()
            .with_state_exit(state)
            .add_system(system);

        self
    }

    fn add_system_set_to_state_update<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static,
    {
        self.world
            .get_resource_mut::<ScheduleStates<S>>()
            .unwrap()
            .with_state_update(state)
            .add_system_set(system_set);

        self
    }

    fn add_system_set_to_state_enter<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static,
    {
        self.world
            .get_resource_mut::<ScheduleStates<S>>()
            .unwrap()
            .with_state_enter(state)
            .add_system_set(system_set);

        self
    }

    fn add_system_set_to_state_exit<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static,
    {
        self.world
            .get_resource_mut::<ScheduleStates<S>>()
            .unwrap()
            .with_state_exit(state)
            .add_system_set(system_set);

        self
    }
}
