# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark      | [ecs](https://github.com/HeroesGrave/ecs-rs)
 -------------- |:--------------------------------------------:
 pos_vel build  | 1,933,665 ns/iter (+/- 35,257)
 pos_vel update | 340,018 ns/iter (+/- 4,800)

### pos_vel
 * 1000 entities with `position` and `velocity` components
 * 9000 entities with `position` components only
 * stub `render` system
 * `physics` system: `position += velocity`
