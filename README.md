# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark       | [ecs](https://github.com/HeroesGrave/ecs-rs) | [specs](https://github.com/slide-rs/specs) | [recs](https://github.com/andybarron/rustic-ecs) | [trex](https://github.com/rcolinray/trex)
 --------------- |:--------------------------------------------:|:------------------------------------------:|:------------------------------------------------:|:-----------------------------------------:
 pos_vel build   | 2,052,168 ns/iter (+/- 70,073)                          | 498,342 ns/iter (+/- 400,352)                      | 19,816,170 ns/iter (+/- 3,132,394)                             | 1,052,629 ns/iter (+/- 271,026)
 pos_vel update  | 439,598 ns/iter (+/- 20,638)                         | 81,890 ns/iter (+/- 28,103)                     | 7,408,965 ns/iter (+/- 1,603,295)                            | 238,207 ns/iter (+/- 5,423)
 parallel build  | 2,005,786 ns/iter (+/- 497,210)                         | 586,433 ns/iter (+/- 168,621)                     | 23,465,808 ns/iter (+/- 2,086,760)                            | 3,173,121 ns/iter (+/- 1,546,373)
 parallel update | 9,711,756 ns/iter (+/- 4,634,118)                        | 104,090 ns/iter (+/- 40,966)                    | 14,161,801 ns/iter (+/- 3,291,843)                           | 500,353 ns/iter (+/- 16,157)

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
