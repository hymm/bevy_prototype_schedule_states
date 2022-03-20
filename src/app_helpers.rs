use bevy::{
    ecs::schedule::IntoSystemDescriptor,
    prelude::{App, SystemSet},
};
use std::hash::Hash;

use crate::ScheduleStates;

pub trait AppStateHelpers {
    fn add_system_to_update<S, Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;

    fn add_system_to_enter<S, Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;

    fn add_system_to_exit<S, Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;

    fn add_system_set_to_update<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;

    fn add_system_set_to_enter<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;

    fn add_system_set_to_exit<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static;

    fn add_nested_driver_to_state<S, T>(&mut self, state: S) -> &mut App
    where
        S: Copy + Clone + Send + Sync + Eq + Hash + 'static,
        T: Eq + Hash + Copy + Send + Sync + 'static;
}

impl AppStateHelpers for App {
    fn add_system_to_update<S, Params>(
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

    fn add_system_to_enter<S, Params>(
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

    fn add_system_to_exit<S, Params>(
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

    fn add_system_set_to_update<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
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

    fn add_system_set_to_enter<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
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

    fn add_system_set_to_exit<S>(&mut self, state: S, system_set: SystemSet) -> &mut App
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

    fn add_nested_driver_to_state<S, T>(&mut self, state: S) -> &mut App
    where
        S: Eq + Hash + Copy + Send + Sync + Hash + 'static,
        T: Eq + Hash + Copy + Send + Sync + 'static,
    {
        self.world
            .get_resource_mut::<ScheduleStates<S>>()
            .unwrap()
            .add_nested_driver_to_state::<T>(state);

        self
    }
}
