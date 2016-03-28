# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

 Benchmark | [ecs-rs](https://github.com/HeroesGrave/ecs-rs)
 --------- |:-----------------------------------------------:
 pos_vel   | 7,118,641 ns

### pos_vel
 * 1000 entities with `position` and `velocity` components
 * 9000 entities with `position` components only
 * stub `render` system
 * `physics` system: `position += velocity`
