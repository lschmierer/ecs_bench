#![feature(test)]

extern crate test;
use test::Bencher;

extern crate froggy;
extern crate ecs_bench;

use froggy::{Pointer, Storage};

use ecs_bench::parallel::{R, W1, W2, N};

struct Entity {
    r: Pointer<R>,
    w1: Pointer<W1>,
    w2: Pointer<W2>,
}

struct World {
    r: Storage<R>,
    w1: Storage<W1>,
    w2: Storage<W2>,
    entities: Vec<Entity>,
}

fn build() -> World {
    let mut world = World {
        r: Storage::with_capacity(N),
        w1: Storage::with_capacity(N),
        w2: Storage::with_capacity(N),
        entities: Vec::with_capacity(N),
    };

    // setup entities
    {
        let mut r = world.r.write();
        let mut w1 = world.w1.write();
        let mut w2 = world.w2.write();

        for _ in 0 .. N {
            world.entities.push(Entity {
                r: r.create(R { x: 0.0 }),
                w1: w1.create(W1 { x: 0.0 }),
                w2: w2.create(W2 { x: 0.0 }),
            });
        }
    }

    world
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    use std::sync::Arc;
    use std::thread;

    let world = Arc::new(build());

    b.iter(|| {
        let wt1 = world.clone();
        let t1 = thread::spawn(move || {
            let r = wt1.r.read();
            let mut w = wt1.w1.write();
            for e in wt1.entities.iter() {
                w.access(&e.w1).x = r.access(&e.r).x;
            }
        });
        let wt2 = world.clone();
        let t2 = thread::spawn(move || {
            let r = wt2.r.read();
            let mut w = wt2.w2.write();
            for e in wt2.entities.iter() {
                w.access(&e.w2).x = r.access(&e.r).x;
            }
        });
        t1.join().unwrap();
        t2.join().unwrap();
    });
}
