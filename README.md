# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 259 µs/iter (+/- 10)      | 16 µs/iter (+/- 0)      | 437 µs/iter (+/- 30)      | 62 µs/iter (+/- 3)
 [constellation] | 252 µs/iter (+/- 14) | 7 µs/iter (+/- 0) | 396 µs/iter (+/- 22) | 97 µs/iter (+/- 33)
 [ecs]           | 1,249 µs/iter (+/- 98)           | 296 µs/iter (+/- 8)           | 1,154 µs/iter (+/- 48)           | 3,648 µs/iter (+/- 142)
 [froggy]        | 611 µs/iter (+/- 19)        | 10 µs/iter (+/- 0)        | 1,472 µs/iter (+/- 115)        | 84 µs/iter (+/- 3)
 [recs]          | {pos_vel_build_recs}          | {pos_vel_update_recs}          | {parallel_build_recs}          | {parallel_update_recs}
 [specs]         | 478 µs/iter (+/- 40)         | 8 µs/iter (+/- 0)         | 709 µs/iter (+/- 37)         | 50 µs/iter (+/- 1)
 [trex]          | 1,581 µs/iter (+/- 73)          | 200 µs/iter (+/- 12)          | 2,116 µs/iter (+/- 118)          | 409 µs/iter (+/- 27)

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
