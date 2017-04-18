# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 299,027 ns/iter (+/- 48,816)      | 19,007 ns/iter (+/- 5,815)      | 437,760 ns/iter (+/- 109,311)      | 77,075 ns/iter (+/- 16,821)
 [constellation] | 306,427 ns/iter (+/- 70,229) | 9,210 ns/iter (+/- 1,777) | 515,732 ns/iter (+/- 91,263) | 164,793 ns/iter (+/- 29,864)
 [ecs]           | 1,402,763 ns/iter (+/- 240,065)           | 293,200 ns/iter (+/- 41,395)           | 1,474,755 ns/iter (+/- 362,815)           | 3,412,194 ns/iter (+/- 600,767)
 [froggy]        | 778,978 ns/iter (+/- 127,929)        | 15,081 ns/iter (+/- 3,146)        | 1,856,722 ns/iter (+/- 412,493)        | 66,002 ns/iter (+/- 21,315)
 [recs]          | 11,407,049 ns/iter (+/- 3,225,550)          | 3,724,867 ns/iter (+/- 918,360)          | 14,532,340 ns/iter (+/- 1,646,550)          | 8,324,179 ns/iter (+/- 2,097,752)
 [specs]         | 297,508 ns/iter (+/- 25,520)         | 45,860 ns/iter (+/- 15,550)         | 399,688 ns/iter (+/- 88,382)         | 57,347 ns/iter (+/- 11,922)
 [trex]          | 950,464 ns/iter (+/- 291,444)          | 228,419 ns/iter (+/- 35,679)          | 1,608,819 ns/iter (+/- 316,586)          | 486,097 ns/iter (+/- 288,601)

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
