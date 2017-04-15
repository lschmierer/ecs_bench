# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark       | [ecs](https://github.com/HeroesGrave/ecs-rs) | [specs](https://github.com/slide-rs/specs) | [recs](https://github.com/andybarron/rustic-ecs) | [trex](https://github.com/rcolinray/trex) | [calx-ecs](https://github.com/rsaarelm/calx-ecs) | [froggy](https://github.com/kvark/froggy)
 --------------- |:--------------------------------------------:|:------------------------------------------:|:------------------------------------------------:|:-----------------------------------------:|:------------------------------------------------:|:-----------------------------------------:
 pos_vel build   | 1,556,468 ns/iter (+/- 202,696)                          | 326,929 ns/iter (+/- 12,156)                      | 14,488,664 ns/iter (+/- 1,097,519)                             | 947,822 ns/iter (+/- 172,427)                      | 1,593,580 ns/iter (+/- 546,347)                         | 873,383 ns/iter (+/- 33,620)
 pos_vel update  | 352,698 ns/iter (+/- 8,469)                         | 91,719 ns/iter (+/- 15,396)                     | 3,766,269 ns/iter (+/- 248,554)                            | 228,796 ns/iter (+/- 4,530)                     | 313,790 ns/iter (+/- 4,708)                        | 16,296 ns/iter (+/- 2,679)
 parallel build  | 1,540,999 ns/iter (+/- 30,858)                         | 465,356 ns/iter (+/- 120,763)                     | 18,450,091 ns/iter (+/- 2,244,547)                            | 1,737,319 ns/iter (+/- 102,261)                     | 2,527,103 ns/iter (+/- 143,438)                        | 2,110,637 ns/iter (+/- 34,339)
 parallel update | 4,276,264 ns/iter (+/- 92,040)                        | 117,415 ns/iter (+/- 20,432)                    | 7,017,418 ns/iter (+/- 1,703,643)                           | 476,917 ns/iter (+/- 6,073)                    | 658,071 ns/iter (+/- 5,546)                       | 135,095 ns/iter (+/- 49,738)

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

## Notes
 * the benchmarks explore a limited subset of ECS use-cases and do not necessarily reflect the peformance of large-scale applications
 * [froggy](https://github.com/kvark/froggy) is technically not an ECS, but a Component Graph System (CGS)
