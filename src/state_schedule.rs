use bevy::{
    prelude::{Mut, Schedule, Stage, StageLabel, SystemStage, World},
    utils::HashMap,
};
use std::hash::Hash;

pub struct ScheduleStates<S>
where
    S: Eq + Hash + Copy + Clone,
{
    pub current_state: S,
    pub next_state: Option<S>,
    first_run: bool,
    enter: HashMap<S, Schedule>,
    update: HashMap<S, Schedule>,
    exit: HashMap<S, Schedule>,
}

#[derive(StageLabel, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct StateStage;

impl<S> ScheduleStates<S>
where
    S: Eq + Hash + Copy,
{
    pub fn new(initial_state: S) -> Self {
        ScheduleStates {
            current_state: initial_state,
            next_state: Some(initial_state),
            first_run: true,
            enter: HashMap::default(),
            update: HashMap::default(),
            exit: HashMap::default(),
        }
    }

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

    pub fn run_update(&mut self, world: &mut World, state: S) {
        self.update.get_mut(&state).unwrap().run(world);
    }

    pub fn run_enter(&mut self, world: &mut World, state: S) {
        self.enter.get_mut(&state).unwrap().run(world);
    }

    pub fn run_exit(&mut self, world: &mut World, state: S) {
        self.exit.get_mut(&state).unwrap().run(world);
    }
}

pub struct NextState<S: Copy>(pub Option<S>);

impl<S: Copy> NextState<S> {
    pub fn set(&mut self, next_state: S) {
        self.0 = Some(next_state);
    }
}

pub fn driver<S>(world: &mut World)
where
    S: Eq + Hash + Copy + Send + Sync + 'static,
{
    world.resource_scope(|world, mut state: Mut<ScheduleStates<S>>| {
        let mut next_state = state.next_state;
        loop {
            if let Some(next_state) = next_state {
                let current_state = state.current_state;
                if !state.first_run {
                  state.run_exit(world, current_state);
                }
                state.current_state = next_state;
                state.next_state = None;
                state.run_enter(world, next_state);
                
                state.first_run = false;
            }
            let current_state = state.current_state;
            state.run_update(world, current_state);

            world.resource_scope(|_world, mut n: Mut<NextState<S>>| {
                next_state = n.0.take();
            });

            if next_state.is_none() {
                break;
            }
        }
    });
}