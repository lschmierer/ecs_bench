#![feature(test)]

extern crate test;
use test::Bencher;

#[macro_use]
extern crate recs;

extern crate ecs_bench;

use recs::{Ecs, EntityId};

use ecs_bench::parallel::{R, W1, W2, N};

fn build() -> Ecs {
    let mut system: Ecs = Ecs::new();

    // setup entities
    for _ in 0..N {
        let ent = system.create_entity();
        let _ = system.set(ent, R { x: 0.0 });
        let _ = system.set(ent, W1 { x: 0.0 });
        let _ = system.set(ent, W2 { x: 0.0 });
    }

    system
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut system = build();

    b.iter(|| {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(R, W1);
        system.collect_with(&filter, &mut ids);
        for id in ids {
            let r = system.get::<R>(id).unwrap();
            let mut w1 = system.borrow_mut::<W1>(id).unwrap();
            w1.x += r.x;
        }

        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(R, W2);
        system.collect_with(&filter, &mut ids);
        for id in ids {
            let r = system.get::<R>(id).unwrap();
            let mut w2 = system.borrow_mut::<W2>(id).unwrap();
            w2.x *= r.x;
        }
    });
}
