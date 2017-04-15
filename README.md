# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark       | [ecs](https://github.com/HeroesGrave/ecs-rs) | [specs](https://github.com/slide-rs/specs) | [recs](https://github.com/andybarron/rustic-ecs) | [trex](https://github.com/rcolinray/trex) | [calx-ecs](https://github.com/rsaarelm/calx-ecs) | [froggy](https://github.com/kvark/froggy)
 --------------- |:--------------------------------------------:|:------------------------------------------:|:------------------------------------------------:|:-----------------------------------------:|:------------------------------------------------:|:-----------------------------------------:
 pos_vel build   | 1,345,684 ns/iter (+/- 16,052)                          | 353,256 ns/iter (+/- 126,738)                      | 13,631,518 ns/iter (+/- 2,166,517)                             | 918,333 ns/iter (+/- 14,290)                      | 303,328 ns/iter (+/- 9,159)                         | 949,397 ns/iter (+/- 44,143)
 pos_vel update  | 369,159 ns/iter (+/- 8,816)                         | 94,785 ns/iter (+/- 15,303)                     | 3,116,631 ns/iter (+/- 160,402)                            | 225,451 ns/iter (+/- 57,390)                     | 18,343 ns/iter (+/- 166)                        | 15,996 ns/iter (+/- 190)
 parallel build  | 1,399,331 ns/iter (+/- 86,013)                         | 454,021 ns/iter (+/- 153,839)                     | 16,956,679 ns/iter (+/- 1,562,317)                            | 1,728,446 ns/iter (+/- 118,019)                     | 446,644 ns/iter (+/- 129,865)                        | 2,073,316 ns/iter (+/- 16,963)
 parallel update | 4,890,021 ns/iter (+/- 71,049)                        | 123,434 ns/iter (+/- 18,972)                    | 6,049,701 ns/iter (+/- 916,957)                           | 487,189 ns/iter (+/- 16,152)                    | 82,541 ns/iter (+/- 1,641)                       | 136,294 ns/iter (+/- 38,945)

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
