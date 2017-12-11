# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 283 µs/iter (+/- 9)      | 20 µs/iter (+/- 0)      | 464 µs/iter (+/- 33)      | 78 µs/iter (+/- 3)
 [constellation] | 311 µs/iter (+/- 10) | 9 µs/iter (+/- 0) | 488 µs/iter (+/- 24) | 169 µs/iter (+/- 15)
 [ecs]           | 1,442 µs/iter (+/- 104)           | 327 µs/iter (+/- 14)           | 1,436 µs/iter (+/- 93)           | 3,823 µs/iter (+/- 200)
 [froggy]        | 645 µs/iter (+/- 36)        | 15 µs/iter (+/- 0)        | 1,493 µs/iter (+/- 62)        | 102 µs/iter (+/- 4)
 [specs]         | 320 µs/iter (+/- 19)         | 4 µs/iter (+/- 0)         | 780 µs/iter (+/- 47)         | 101 µs/iter (+/- 6)
 [trex]          | 1,194 µs/iter (+/- 88)          | 203 µs/iter (+/- 11)          | 1,685 µs/iter (+/- 153)          | 425 µs/iter (+/- 18)

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
