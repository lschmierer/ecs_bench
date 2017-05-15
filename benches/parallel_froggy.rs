#![feature(test)]

extern crate test;
use test::Bencher;

extern crate froggy;
extern crate ecs_bench;

use froggy::{Pointer, Storage};
use std::sync::{Arc, Mutex};

use ecs_bench::parallel::{R, W1, W2, N};

struct Entity {
    r: Pointer<R>,
    w1: Pointer<W1>,
    w2: Pointer<W2>,
}

struct World {
    r: Storage<R>,
    w1: Mutex<Storage<W1>>,
    w2: Mutex<Storage<W2>>,
    entities: Vec<Entity>,
}

fn build() -> World {
    let mut r = Storage::with_capacity(N);
    let mut w1 = Storage::with_capacity(N);
    let mut w2 = Storage::with_capacity(N);

    // setup entities
    let entities = (0 .. N).map(|_| Entity {
        r: r.create(R { x: 0.0 }),
        w1: w1.create(W1 { x: 0.0 }),
        w2: w2.create(W2 { x: 0.0 }),
    }).collect();

    // finish processing
    r.sync_pending();
    w1.sync_pending();
    w2.sync_pending();

    World {
        r: r,
        w1: Mutex::new(w1),
        w2: Mutex::new(w2),
        entities: entities,
    }
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    use std::thread;

    let world = Arc::new(build());

    b.iter(|| {
        let wt1 = world.clone();
        let t1 = thread::spawn(move || {
            let mut w1 = wt1.w1.lock().unwrap();
            for e in &wt1.entities {
                w1[&e.w1].x = wt1.r[&e.r].x;
            }
        });
        let wt2 = world.clone();
        let t2 = thread::spawn(move || {
            let mut w2 = wt2.w2.lock().unwrap();
            for e in &wt2.entities {
                w2[&e.w2].x = wt2.r[&e.r].x;
            }
        });
        t1.join().unwrap();
        t2.join().unwrap();
    });
}
