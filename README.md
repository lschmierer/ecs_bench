# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark       | [ecs]                 | [specs]                 | [recs]                 | [trex]                 | [calx-ecs]                 | [froggy]                 | [constellation]
 --------------- |:---------------------:|:-----------------------:|:----------------------:|:----------------------:|:--------------------------:|:------------------------:|:--------------------------------:
 pos_vel build   | 1,400,175 ns/iter (+/- 219,232)   | 299,432 ns/iter (+/- 21,723)   | 11,531,612 ns/iter (+/- 1,653,725)   | 943,169 ns/iter (+/- 174,093)   | 306,408 ns/iter (+/- 49,593)   | 787,266 ns/iter (+/- 164,335)   | 293,114 ns/iter (+/- 43,732)
 pos_vel update  | 296,750 ns/iter (+/- 52,060)  | 41,380 ns/iter (+/- 6,439)  | 3,852,468 ns/iter (+/- 969,110)  | 219,278 ns/iter (+/- 40,183)  | 19,123 ns/iter (+/- 5,586)  | 14,661 ns/iter (+/- 2,449)  | 9,019 ns/iter (+/- 1,682)
 parallel build  | 1,449,814 ns/iter (+/- 312,318)  | 396,730 ns/iter (+/- 40,184)  | 14,189,162 ns/iter (+/- 2,249,327)  | 1,615,398 ns/iter (+/- 273,796)  | 442,755 ns/iter (+/- 88,803)  | 1,910,969 ns/iter (+/- 348,899)  | 512,111 ns/iter (+/- 351,107)
 parallel update | 3,390,745 ns/iter (+/- 780,609) | 57,026 ns/iter (+/- 1,840) | 8,579,522 ns/iter (+/- 1,737,264) | 521,482 ns/iter (+/- 93,310) | 76,538 ns/iter (+/- 17,799) | 65,100 ns/iter (+/- 5,861) | 164,824 ns/iter (+/- 29,927)

[ecs]: https://github.com/HeroesGrave/ecs-rs
[specs]: https://github.com/slide-rs/specs
[recs]: https://github.com/andybarron/rustic-ecs
[trex]: https://github.com/rcolinray/trex
[calx-ecs]: https://github.com/rsaarelm/calx-ecs
[froggy]: https://github.com/kvark/froggy
[constellation]: https://github.com/TomGillen/constellation/

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
