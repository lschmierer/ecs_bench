# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 288 µs/iter (+/- 29)      | 18 µs/iter (+/- 1)      | 505 µs/iter (+/- 234)      | 67 µs/iter (+/- 18)
 [constellation] | 271 µs/iter (+/- 31) | 8 µs/iter (+/- 6) | 452 µs/iter (+/- 132) | 159 µs/iter (+/- 235)
 [ecs]           | 1,422 µs/iter (+/- 645)           | 312 µs/iter (+/- 144)           | 1,567 µs/iter (+/- 733)           | 3,899 µs/iter (+/- 1,455)
 [froggy]        | 685 µs/iter (+/- 271)        | 10 µs/iter (+/- 1)        | 1,601 µs/iter (+/- 597)        | 99 µs/iter (+/- 40)
 [specs]         | 479 µs/iter (+/- 60)         | 9 µs/iter (+/- 4)         | 797 µs/iter (+/- 123)         | 52 µs/iter (+/- 6)
 [trex]          | 1,875 µs/iter (+/- 757)          | 222 µs/iter (+/- 140)          | 2,310 µs/iter (+/- 248)          | 422 µs/iter (+/- 50)

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
