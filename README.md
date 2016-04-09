# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark      | [ecs](https://github.com/HeroesGrave/ecs-rs) | [parsec](https://github.com/kvark/parsec)
 -------------- |:--------------------------------------------:|:-----------------------------------------:
 pos_vel build  | 2,042,861 ns/iter (+/- 55,533)               | 1,670,728 ns/iter (+/- 372,800)
 pos_vel update | 406,616 ns/iter (+/- 31,342)                 | 329,843 ns/iter (+/- 47,088)

### pos_vel
 * 1000 entities with `position` and `velocity` components
 * 9000 entities with `position` components only
 * stub `render` system
 * `physics` system: `position += velocity`
