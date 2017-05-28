# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 302,332 ns/iter (+/- 2,990)      | 18,386 ns/iter (+/- 250)      | 440,874 ns/iter (+/- 6,423)      | 79,409 ns/iter (+/- 547)
 [constellation] | 252,529 ns/iter (+/- 11,599) | 7,729 ns/iter (+/- 56) | 480,754 ns/iter (+/- 14,587) | 319,724 ns/iter (+/- 82,996)
 [ecs]           | 1,376,926 ns/iter (+/- 38,501)           | 355,844 ns/iter (+/- 7,622)           | 1,408,404 ns/iter (+/- 26,887)           | 4,612,026 ns/iter (+/- 27,155)
 [froggy]        | 1,188,280 ns/iter (+/- 11,236)        | 21,173 ns/iter (+/- 329)        | 2,837,011 ns/iter (+/- 46,421)        | 144,532 ns/iter (+/- 45,930)
 [recs]          | 12,714,400 ns/iter (+/- 5,416,407)          | 3,334,991 ns/iter (+/- 1,116,488)          | 15,321,184 ns/iter (+/- 1,435,306)          | 5,562,233 ns/iter (+/- 394,955)
 [specs]         | 330,778 ns/iter (+/- 15,790)         | 91,364 ns/iter (+/- 13,819)         | 427,871 ns/iter (+/- 46,685)         | 122,613 ns/iter (+/- 27,909)
 [trex]          | 931,601 ns/iter (+/- 6,821)          | 199,088 ns/iter (+/- 1,758)          | 1,787,754 ns/iter (+/- 148,991)          | 442,316 ns/iter (+/- 18,376)

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
