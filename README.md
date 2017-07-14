# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 285 µs/iter (+/- 10)      | 20 µs/iter (+/- 0)      | 493 µs/iter (+/- 50)      | 74 µs/iter (+/- 5)
 [constellation] | 307 µs/iter (+/- 7) | 8 µs/iter (+/- 0) | 471 µs/iter (+/- 9) | 166 µs/iter (+/- 2)
 [ecs]           | 1,698 µs/iter (+/- 132)           | 366 µs/iter (+/- 100)           | 1,482 µs/iter (+/- 27)           | 3,610 µs/iter (+/- 55)
 [froggy]        | 662 µs/iter (+/- 6)        | 14 µs/iter (+/- 0)        | 1,521 µs/iter (+/- 32)        | 104 µs/iter (+/- 12)
 [specs]         | 463 µs/iter (+/- 31)         | 13 µs/iter (+/- 0)         | 760 µs/iter (+/- 83)         | 99 µs/iter (+/- 9)
 [trex]          | 1,439 µs/iter (+/- 256)          | 198 µs/iter (+/- 12)          | 2,146 µs/iter (+/- 99)          | 422 µs/iter (+/- 11)

[calx-ecs]: https://github.com/rsaarelm/calx-ecs
[constellation]: https://github.com/TomGillen/constellation/
[ecs]: https://github.com/HeroesGrave/ecs-rs
[froggy]: https://github.com/kvark/froggy
[specs]: https://github.com/slide-rs/specs
[trex]: https://github.com/rcolinray/trex


Visualization of benchmarks, smaller is better.
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
