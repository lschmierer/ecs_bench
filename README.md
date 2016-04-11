# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark       | [ecs](https://github.com/HeroesGrave/ecs-rs) | [parsec](https://github.com/kvark/parsec)
 --------------- |:--------------------------------------------:|:-----------------------------------------:
 pos_vel build   | 2,214,413 ns/iter (+/- 850,677)              | 279,377 ns/iter (+/- 126,655)
 pos_vel update  | 399,810 ns/iter (+/- 7,190)                  | 263,236 ns/iter (+/- 5,039)
 parallel build  | 1,873,907 ns/iter (+/- 925,296)              | 603,136 ns/iter (+/- 136,539)
 parallel update | 5,119,899 ns/iter (+/- 4,130,840)            | 269,848 ns/iter (+/- 167,807)

### pos_vel
 * 1000 entities with `position` and `velocity` components
 * 9000 entities with `position` components only
 * stub `render` system
 * `physics` system: `position += velocity`

### parallel
 * 10000 entities with 3 simple components `R`, `W1` and `W2`
 * `w1` system reads `R` and writes to `W1`
 * `w2` system reads `R` and writes to `W2`
