# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 259,498 ns/iter (+/- 14,073)      | 16,923 ns/iter (+/- 625)      | 468,736 ns/iter (+/- 18,206)      | 61,278 ns/iter (+/- 3,564)
 [constellation] | 247,096 ns/iter (+/- 14,133) | 7,394 ns/iter (+/- 281) | 394,240 ns/iter (+/- 33,396) | 92,023 ns/iter (+/- 44,463)
 [ecs]           | 1,204,888 ns/iter (+/- 92,548)           | 284,273 ns/iter (+/- 8,012)           | 1,202,208 ns/iter (+/- 63,634)           | 3,628,299 ns/iter (+/- 189,770)
 [froggy]        | 621,384 ns/iter (+/- 56,370)        | 9,922 ns/iter (+/- 366)        | 1,403,936 ns/iter (+/- 75,464)        | 85,482 ns/iter (+/- 6,672)
 [recs]          | 5,165,557 ns/iter (+/- 860,578)          | 3,644,667 ns/iter (+/- 1,116,716)          | 12,573,088 ns/iter (+/- 1,580,716)          | 9,103,369 ns/iter (+/- 3,317,322)
 [specs]         | 471,536 ns/iter (+/- 24,807)         | 8,868 ns/iter (+/- 381)         | 752,958 ns/iter (+/- 127,086)         | 46,947 ns/iter (+/- 695)
 [trex]          | 1,693,309 ns/iter (+/- 149,352)          | 193,135 ns/iter (+/- 19,970)          | 2,098,170 ns/iter (+/- 236,378)          | 389,157 ns/iter (+/- 17,623)

[calx-ecs]: https://github.com/rsaarelm/calx-ecs
[constellation]: https://github.com/TomGillen/constellation/
[ecs]: https://github.com/HeroesGrave/ecs-rs
[froggy]: https://github.com/kvark/froggy
[recs]: https://github.com/andybarron/rustic-ecs
[specs]: https://github.com/slide-rs/specs
[trex]: https://github.com/rcolinray/trex

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
