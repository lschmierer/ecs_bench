# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark       | [ecs](https://github.com/HeroesGrave/ecs-rs) | [specs](https://github.com/slide-rs/specs) | [recs](https://github.com/andybarron/rustic-ecs) | [trex](https://github.com/rcolinray/trex)
 --------------- |:--------------------------------------------:|:------------------------------------------:|:------------------------------------------------:|:-----------------------------------------:
 pos_vel build   | 2,798,862 ns/iter (+/- 927,802)                          | 518,907 ns/iter (+/- 77,022)                      | 28,653,720 ns/iter (+/- 11,130,826)                             | 1,795,407 ns/iter (+/- 849,279)
 pos_vel update  | 786,643 ns/iter (+/- 336,738)                         | 86,353 ns/iter (+/- 45,238)                     | 11,205,047 ns/iter (+/- 4,815,837)                            | 339,966 ns/iter (+/- 184,855)
 parallel build  | 2,829,295 ns/iter (+/- 387,589)                         | 789,047 ns/iter (+/- 94,704)                     | 33,180,037 ns/iter (+/- 14,750,948)                            | 3,296,821 ns/iter (+/- 1,651,951)
 parallel update | 9,315,261 ns/iter (+/- 4,727,750)                        | 142,467 ns/iter (+/- 26,575)                    | 20,871,124 ns/iter (+/- 5,495,705)                           | 864,386 ns/iter (+/- 100,686)

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
