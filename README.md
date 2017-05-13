# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 302,402 ns/iter (+/- 12,047)      | 18,425 ns/iter (+/- 209)      | 440,487 ns/iter (+/- 7,675)      | 80,048 ns/iter (+/- 1,751)
 [constellation] | 253,822 ns/iter (+/- 23,158) | 7,836 ns/iter (+/- 236) | 459,891 ns/iter (+/- 9,507) | 293,281 ns/iter (+/- 136,014)
 [ecs]           | 1,629,996 ns/iter (+/- 513,107)           | 388,352 ns/iter (+/- 124,691)           | 1,393,983 ns/iter (+/- 20,981)           | 4,265,622 ns/iter (+/- 78,579)
 [froggy]        | 884,746 ns/iter (+/- 63,404)        | 16,185 ns/iter (+/- 543)        | 2,102,770 ns/iter (+/- 277,499)        | 147,294 ns/iter (+/- 72,925)
 [recs]          | 14,452,607 ns/iter (+/- 3,599,347)          | 3,410,802 ns/iter (+/- 1,532,849)          | 19,607,071 ns/iter (+/- 2,567,083)          | 7,056,556 ns/iter (+/- 4,563,801)
 [specs]         | 372,181 ns/iter (+/- 80,645)         | 100,205 ns/iter (+/- 27,512)         | 488,873 ns/iter (+/- 127,017)         | 130,782 ns/iter (+/- 36,891)
 [trex]          | 938,872 ns/iter (+/- 14,951)          | 225,835 ns/iter (+/- 151,802)          | 1,800,083 ns/iter (+/- 725,044)          | 487,405 ns/iter (+/- 30,319)

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
