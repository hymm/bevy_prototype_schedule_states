mod app_helpers;
mod state_schedule;
use std::{hash::Hash, marker::PhantomData};

pub use app_helpers::AppStateHelpers;
use bevy::prelude::{App, Plugin};
pub use state_schedule::{driver, NextState, ScheduleStates};
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
        app.insert_resource(NextState::<S>(None))
            .insert_resource(ScheduleStates::<S>::new(self.initial_state));
    }
}
