# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark       | [ecs](https://github.com/HeroesGrave/ecs-rs) | [parsec](https://github.com/kvark/parsec)
 --------------- |:--------------------------------------------:|:-----------------------------------------:
 pos_vel build   | 2,871,544 ns/iter (+/- 765,699)              | 499,714 ns/iter (+/- 149,652)
 pos_vel update  | 769,322 ns/iter (+/- 51,116)                 | 355,962 ns/iter (+/- 34,340)
 parallel build  | 2,749,608 ns/iter (+/- 390,895)              | 705,178 ns/iter (+/- 119,296)
 parallel update | 9,211,415 ns/iter (+/- 2,295,994)            | 361,683 ns/iter (+/- 97,809)

### pos_vel
 * 1000 entities with `position` and `velocity` components
 * 9000 entities with `position` components only
 * stub `render` system
 * `physics` system: `position += velocity`

### parallel
 * 10000 entities with 3 simple components `R`, `W1` and `W2`
 * `w1` system reads `R` and writes to `W1`
 * `w2` system reads `R` and writes to `W2`
