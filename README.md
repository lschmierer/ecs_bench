# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark       | [ecs](https://github.com/HeroesGrave/ecs-rs) | [specs](https://github.com/slide-rs/specs) | [recs](https://github.com/andybarron/rustic-ecs) | [trex](https://github.com/rcolinray/trex)
 --------------- |:--------------------------------------------:|:------------------------------------------:|:------------------------------------------------:|:-----------------------------------------:
 pos_vel build   | 1,951,491 ns/iter (+/- 857,043)              | 670,377 ns/iter (+/- 512,758)              | 18,996,823 ns/iter (+/- 1,069,643)               | 996,287 ns/iter (+/- 60,093)
 pos_vel update  | 412,365 ns/iter (+/- 8,586)                  | 298,299 ns/iter (+/- 49,538)               | 7,919,529 ns/iter (+/- 1,925,982)                | 227,512 ns/iter (+/- 3,805)
 parallel build  | 1,895,539 ns/iter (+/- 58,357)               | 697,000 ns/iter (+/- 255,898)              | 37,157,731 ns/iter (+/- 2,366,647)               | 1,875,981 ns/iter (+/- 155,118)
 parallel update | 5,024,416 ns/iter (+/- 1,512,566)            | 351,186 ns/iter (+/- 94,830)               | 15,971,093 ns/iter (+/- 2,943,492)               | 480,172 ns/iter (+/- 102,786)

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
