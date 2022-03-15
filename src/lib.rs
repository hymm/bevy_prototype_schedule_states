mod state_schedule;
use std::{hash::Hash, marker::PhantomData};

use bevy::prelude::{App, Plugin};
pub use state_schedule::{driver, NextState, ScheduleStates};

pub struct StatePlugin<S> {
    _o: PhantomData<S>,
}

impl<S> Default for StatePlugin<S> {
    fn default() -> Self {
        Self { _o: PhantomData }
    }
}

impl<S> Plugin for StatePlugin<S>
where
    S: Send + Sync + 'static + Eq + Hash + Copy + Clone,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(NextState::<S>(None));
    }
}
