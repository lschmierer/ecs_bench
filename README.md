# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark       | [ecs](https://github.com/HeroesGrave/ecs-rs) | [parsec](https://github.com/kvark/parsec) | [recs](https://github.com/andybarron/rustic-ecs)
 --------------- |:--------------------------------------------:|:-----------------------------------------:|:-------------------------------------:
 pos_vel build   | 2,013,306 ns/iter (+/- 788,473)              | 578,499 ns/iter (+/- 253,662)             | 20,182,358 ns/iter (+/- 11,517,429)
 pos_vel update  | 414,978 ns/iter (+/- 27,808)                 | 341,488 ns/iter (+/- 29,824)              | 8,020,501 ns/iter (+/- 3,332,460)
 parallel build  | 1,890,371 ns/iter (+/- 56,579)               | 728,059 ns/iter (+/- 200,764)             | 22,702,370 ns/iter (+/- 5,162,507)
 parallel update | 5,192,265 ns/iter (+/- 590,865)              | 270,586 ns/iter (+/- 53,339)              | 16,014,551 ns/iter (+/- 7,606,844)

### pos_vel
 * 1000 entities with `position` and `velocity` components
 * 9000 entities with `position` components only
 * stub `render` system
 * `physics` system: `position += velocity`

### parallel
 * 10000 entities with 3 simple components `R`, `W1` and `W2`
 * `w1` system reads `R` and writes to `W1`
 * `w2` system reads `R` and writes to `W2`
 * systems could be run in parallel
