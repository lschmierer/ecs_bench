# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Benchmark       | [ecs](https://github.com/HeroesGrave/ecs-rs) | [specs](https://github.com/slide-rs/specs) | [recs](https://github.com/andybarron/rustic-ecs) | [trex](https://github.com/rcolinray/trex) | [calx-ecs](https://github.com/rsaarelm/calx-ecs)
 --------------- |:--------------------------------------------:|:------------------------------------------:|:------------------------------------------------:|:-----------------------------------------:|:-----------------------------------------:
 pos_vel build   | 1,491,014 ns/iter (+/- 34,131)                          | 353,478 ns/iter (+/- 11,519)                      | 12,991,245 ns/iter (+/- 1,171,614)                             | 905,671 ns/iter (+/- 12,623)                      | 1,446,631 ns/iter (+/- 33,135)
 pos_vel update  | 419,477 ns/iter (+/- 5,409)                         | 88,313 ns/iter (+/- 14,083)                     | 3,746,524 ns/iter (+/- 268,436)                            | 228,335 ns/iter (+/- 5,721)                     | 317,819 ns/iter (+/- 3,200)
 parallel build  | 1,506,556 ns/iter (+/- 30,618)                         | 440,786 ns/iter (+/- 5,975)                     | 14,925,527 ns/iter (+/- 604,125)                            | 1,683,709 ns/iter (+/- 94,037)                     | 2,563,698 ns/iter (+/- 56,981)
 parallel update | 5,742,808 ns/iter (+/- 54,541)                        | 105,229 ns/iter (+/- 11,373)                    | 6,341,755 ns/iter (+/- 155,514)                           | 474,164 ns/iter (+/- 4,928)                    | 659,168 ns/iter (+/- 1,620)

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
