use bevy::{
    ecs::schedule::IntoSystemDescriptor,
    prelude::{Schedule, Stage, StageLabel, SystemStage, World, Mut},
    utils::HashMap,
};
use std::hash::Hash;

pub struct ScheduleStates<S>
where
    S: Eq + Hash + Copy,
{
    pub current_state: S,
    pub next_state: Option<S>,
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
            enter: HashMap::default(),
            update: HashMap::default(),
            exit: HashMap::default(),
        }
    }

    pub fn add_state(&mut self, new_state: S) {
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

    pub fn on_enter_with_system<Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) {
        self.enter
            .get_mut(&state)
            .unwrap()
            .add_system_to_stage(StateStage, system);
    }

    pub fn on_update_with_system<Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) {
        self.update
            .get_mut(&state)
            .unwrap()
            .add_system_to_stage(StateStage, system);
    }

    pub fn on_exit_with_system<Params>(
        &mut self,
        state: S,
        system: impl IntoSystemDescriptor<Params>,
    ) {
        self.exit
            .get_mut(&state)
            .unwrap()
            .add_system_to_stage(StateStage, system);
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

struct NextState<S: Copy>(pub Option<S>);

pub fn driver<S>(world: &mut World) 
where
    S: Eq + Hash + Copy + Send + Sync + 'static,
{
    world.resource_scope(|world, mut state: Mut<ScheduleStates<S>>| {
        
        let mut next_state = state.next_state;
        loop {
            if let Some(next_state) = next_state {
                if state.current_state != next_state {
                    let current_state = state.current_state;
                    state.run_exit(world, current_state);
                    state.current_state = next_state;
                    state.next_state = None;
                    state.run_enter(world, next_state);
                }
            }
            let current_state = state.current_state;
            state.run_update(world, current_state);

            world.resource_scope(|_world, mut n: Mut<Option<NextState<S>>>| {
                if let Some(n) = &mut *n {
                    next_state = n.0.take();
                }
            });

            if next_state.is_none() {
                break;
            }
        }
        
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
