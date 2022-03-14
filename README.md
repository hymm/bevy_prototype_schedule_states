# bevy_prototype_schedule_states

This library is a state abstraction for bevy that is compatible with the built-in `FixedTimestep`. This library works by giving each state enter, update, and exit `Schedule` and running these schedules from inside an exclusive system. This means that you will lose parallelism between systems that in the state and systems that are not part of the state.

## Usage

See example in repo.