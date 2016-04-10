# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark      | [ecs](https://github.com/HeroesGrave/ecs-rs) | [parsec](https://github.com/kvark/parsec)
 -------------- |:--------------------------------------------:|:-----------------------------------------:
 pos_vel build  | 1,943,329 ns/iter (+/- 31,943)               | 499,311 ns/iter (+/- 478,846)
 pos_vel update | 404,079 ns/iter (+/- 18,055)                 | 244,365 ns/iter (+/- 16,294)

### pos_vel
 * 1000 entities with `position` and `velocity` components
 * 9000 entities with `position` components only
 * stub `render` system
 * `physics` system: `position += velocity`
