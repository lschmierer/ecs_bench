# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 262,246 ns/iter (+/- 10,243)      | 20,875 ns/iter (+/- 11,458)      | 455,406 ns/iter (+/- 129,556)      | 91,428 ns/iter (+/- 53,627)
 [constellation] | 326,545 ns/iter (+/- 113,917) | 8,861 ns/iter (+/- 4,076) | 489,986 ns/iter (+/- 332,728) | 349,775 ns/iter (+/- 137,372)
 [ecs]           | 1,372,789 ns/iter (+/- 17,618)           | 340,118 ns/iter (+/- 7,066)           | 1,534,586 ns/iter (+/- 812,866)           | 4,864,300 ns/iter (+/- 960,516)
 [froggy]        | 618,024 ns/iter (+/- 35,137)        | 14,562 ns/iter (+/- 113)        | 1,418,492 ns/iter (+/- 283,388)        | 122,803 ns/iter (+/- 86,061)
 [recs]          | 10,282,752 ns/iter (+/- 1,900,688)          | 3,402,040 ns/iter (+/- 782,486)          | 14,629,164 ns/iter (+/- 4,185,851)          | 5,840,296 ns/iter (+/- 659,755)
 [specs]         | 1,329,521 ns/iter (+/- 975,959)         | 99,369 ns/iter (+/- 22,322)         | 2,509,879 ns/iter (+/- 1,876,549)         | 167,202 ns/iter (+/- 21,261)
 [trex]          | 1,065,914 ns/iter (+/- 490,157)          | 213,262 ns/iter (+/- 3,674)          | 1,517,165 ns/iter (+/- 127,046)          | 438,299 ns/iter (+/- 42,209)

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
