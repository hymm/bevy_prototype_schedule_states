use bevy::{
    prelude::{IntoExclusiveSystem, Mut, Schedule, Stage, StageLabel, SystemStage, World},
    utils::HashMap,
};
use std::hash::Hash;

/// `StageLabel` of `Stage` for `ScheduleStatea` to insert systems into
#[derive(StageLabel, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct StateStage;

/// Resource of `Schedule`s attached to states
pub struct ScheduleStates<S>
where
    S: Eq + Hash + Copy + Clone,
{
    current_state: S,
    first_run: bool,
    enter: HashMap<S, Schedule>,
    update: HashMap<S, Schedule>,
    exit: HashMap<S, Schedule>,
}

impl<S> ScheduleStates<S>
where
    S: Eq + Hash + Copy,
{
    /// creates a new `ScheduleStates` with an `initial_state`
    pub fn new(initial_state: S) -> Self {
        ScheduleStates {
            current_state: initial_state,
            first_run: true,
            enter: HashMap::default(),
            update: HashMap::default(),
            exit: HashMap::default(),
        }
    }

    /// get the current state
    pub fn current_state(&self) -> S {
        self.current_state
    }

    /// adds schedules for a new state
    fn add_state(&mut self, new_state: S) {
        let mut schedule = Schedule::default();
        schedule.add_stage(StateStage, SystemStage::parallel());
        self.enter.insert(new_state, schedule);

        let mut schedule = Schedule::default();
        schedule.add_stage(StateStage, SystemStage::parallel());
        self.update.insert(new_state, schedule);

        let mut schedule = Schedule::default();
        schedule.add_stage(StateStage, SystemStage::parallel());
        self.exit.insert(new_state, schedule);
    }

    /// gets the `Schedule` associated with entering `state`. This is used for adding
    /// new systems to that schedule.
    pub fn with_state_enter(&mut self, state: S) -> &mut SystemStage {
        if self.enter.get(&state).is_none() {
            self.add_state(state);
        }
        self.enter
            .get_mut(&state)
            .unwrap()
            .get_stage_mut::<SystemStage>(&StateStage)
            .unwrap()
    }

    /// gets the `Schedule` associated with updating `state`. This is used for adding
    /// new systems to that schedule.
    pub fn with_state_update(&mut self, state: S) -> &mut SystemStage {
        if self.update.get(&state).is_none() {
            self.add_state(state);
        }
        self.update
            .get_mut(&state)
            .unwrap()
            .get_stage_mut::<SystemStage>(&StateStage)
            .unwrap()
    }

    /// gets the `Schedule` associated with exiting `state`. This is used for adding
    /// new systems to that schedule.
    pub fn with_state_exit(&mut self, state: S) -> &mut SystemStage {
        if self.exit.get(&state).is_none() {
            self.add_state(state);
        }
        self.exit
            .get_mut(&state)
            .unwrap()
            .get_stage_mut::<SystemStage>(&StateStage)
            .unwrap()
    }

    /// runs the update `Schedule` associated with state `S`
    pub fn run_update(&mut self, world: &mut World, state: S) {
        self.update.get_mut(&state).unwrap().run(world);
    }

    /// runs the entering `Schedule` associated with state `S`
    pub fn run_enter(&mut self, world: &mut World, state: S) {
        self.enter.get_mut(&state).unwrap().run(world);
    }

    /// runs the exiting `Schedule` associated with state `S`
    pub fn run_exit(&mut self, world: &mut World, state: S) {
        self.exit.get_mut(&state).unwrap().run(world);
    }

    /// add driver for states `T` to state `S`. This adds the
    /// systems responsible for running the correct enter and exit
    /// systems
    pub fn add_nested_driver_to_state<T>(&mut self, state: S)
    where
        T: Eq + Hash + Copy + Send + Sync + 'static,
    {
        self.with_state_enter(state)
            .add_system(driver_run_enter::<T>.exclusive_system());
        self.with_state_update(state)
            .add_system(driver::<T>.exclusive_system());
        self.with_state_exit(state)
            .add_system(driver_run_exit::<T>.exclusive_system());
    }
}

/// Resource for queuing a state change. Only one state can
/// be queued at a time. If another state is queued then it will
/// overwrite the previously queued state.
pub struct NextState<S: Copy>(Option<S>);

impl<S: Copy> Default for NextState<S> {
    fn default() -> Self {
        NextState(None)
    }
}

impl<S: Copy> NextState<S> {
    pub fn set(&mut self, next_state: S) {
        self.0 = Some(next_state);
    }
}

/// The system responsible for running the state schedules.
pub fn driver<S>(world: &mut World)
where
    S: Eq + Hash + Copy + Send + Sync + 'static,
{
    let mut next_state = None;
    world.resource_scope(|_world, mut n: Mut<NextState<S>>| {
        next_state = n.0.take();
    });

    world.resource_scope(|world, mut state: Mut<ScheduleStates<S>>| loop {
        if let Some(next_state) = next_state {
            let current_state = state.current_state;
            state.run_exit(world, current_state);
            state.current_state = next_state;
            state.run_enter(world, next_state);
            state.first_run = false;
        }

        let current_state = state.current_state;
        // TODO: check if this might be a bit buggy if a user queue a next state before the first run of the driver
        if state.first_run {
            state.first_run = false;
            state.run_enter(world, current_state);
        }

        state.run_update(world, current_state);

        // check if the state queued a new state internally
        world.resource_scope(|_world, mut n: Mut<NextState<S>>| {
            next_state = n.0.take();
        });

        if next_state.is_none() {
            break;
        }
    });
}

/// A exclusive system that will run the enter schedule for the current state.
/// Used by nested states.
pub fn driver_run_enter<S>(world: &mut World)
where
    S: Eq + Hash + Copy + Send + Sync + 'static,
{
    world.resource_scope(|world, mut state: Mut<ScheduleStates<S>>| {
        let current_state = state.current_state();
        state.run_enter(world, current_state);
    });
}

/// A exclusive system that will run the exit schedule for the current state.
///Used by nested states.
pub fn driver_run_exit<S>(world: &mut World)
where
    S: Eq + Hash + Copy + Send + Sync + 'static,
{
    world.resource_scope(|world, mut state: Mut<ScheduleStates<S>>| {
        let current_state = state.current_state();
        state.run_exit(world, current_state);
    });
}
