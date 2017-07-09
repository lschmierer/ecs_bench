# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 247 µs/iter (+/- 14)      | 17 µs/iter (+/- 0)      | 437 µs/iter (+/- 21)      | 62 µs/iter (+/- 6)
 [constellation] | 248 µs/iter (+/- 32) | 7 µs/iter (+/- 0) | 411 µs/iter (+/- 96) | 96 µs/iter (+/- 59)
 [ecs]           | 1,213 µs/iter (+/- 107)           | 292 µs/iter (+/- 24)           | 1,223 µs/iter (+/- 106)           | 3,708 µs/iter (+/- 407)
 [froggy]        | 632 µs/iter (+/- 58)        | 10 µs/iter (+/- 0)        | 1,500 µs/iter (+/- 262)        | 86 µs/iter (+/- 13)
 [recs]          | {pos_vel_build_recs}          | {pos_vel_update_recs}          | {parallel_build_recs}          | {parallel_update_recs}
 [specs]         | 467 µs/iter (+/- 37)         | 9 µs/iter (+/- 2)         | 733 µs/iter (+/- 121)         | 47 µs/iter (+/- 5)
 [trex]          | 1,711 µs/iter (+/- 215)          | 199 µs/iter (+/- 8)          | 2,142 µs/iter (+/- 221)          | 400 µs/iter (+/- 34)

[calx-ecs]: https://github.com/rsaarelm/calx-ecs
[constellation]: https://github.com/TomGillen/constellation/
[ecs]: https://github.com/HeroesGrave/ecs-rs
[froggy]: https://github.com/kvark/froggy
[recs]: https://github.com/andybarron/rustic-ecs
[specs]: https://github.com/slide-rs/specs
[trex]: https://github.com/rcolinray/trex

![update benchmarks graph](./graph/update.png)
![build benchmarks graph](./graph/build.png)

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
