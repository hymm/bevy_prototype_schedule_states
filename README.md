# bevy_prototype_schedule_states

This library is a experimental state abstraction for Bevy that is compatible with the built-in `FixedTimestep`. This library works by giving each state enter, update, and exit a `Schedule` and runs these schedules from inside an exclusive system.

## Comparison to Bevy's Builtin State API

### Pros

* Does not use run criteria, so is compatible with Bevy's built-in fixed timestep.
* Simpler, so there is less surprising behavior.
* Can nest states for more complex states.

### Cons

* There is no stack so cannot run multiple states at once.
* Potentially less parallelism between systems since it's built on looping inside an exclusive system instead of run criteria.

## Usage

See examples in repo.
