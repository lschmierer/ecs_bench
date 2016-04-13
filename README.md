# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark       | [ecs](https://github.com/HeroesGrave/ecs-rs) | [specs](https://github.com/slide-rs/specs) | [recs](https://github.com/andybarron/rustic-ecs)
 --------------- |:--------------------------------------------:|:------------------------------------------:|:-------------------------------------:
 pos_vel build   | 1,924,862 ns/iter (+/- 37,504)               | 535,620 ns/iter (+/- 347,990)              | 18,388,538 ns/iter (+/- 1,739,171)
 pos_vel update  | 413,979 ns/iter (+/- 7,523)                  | 320,826 ns/iter (+/- 57,644)               | 6,634,478 ns/iter (+/- 1,119,110)
 parallel build  | 2,707,027 ns/iter (+/- 297,971)              | 556,668 ns/iter (+/- 247,153)              | 22,492,356 ns/iter (+/- 5,866,024)
 parallel update | 5,060,735 ns/iter (+/- 48,238)               | 222,959 ns/iter (+/- 58,226)               | 13,577,225 ns/iter (+/- 4,045,761)

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
