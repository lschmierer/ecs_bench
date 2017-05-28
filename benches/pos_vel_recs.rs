#![feature(test)]

extern crate test;
use test::Bencher;

#[macro_use]
extern crate recs;

extern crate ecs_bench;

use recs::{Ecs, EntityId};

use ecs_bench::pos_vel::{Position, Velocity, N_POS_PER_VEL, N_POS};

fn build() -> Ecs {
    let mut system: Ecs = Ecs::new();

    // setup entities
    for i in 0..N_POS {
        let ent = system.create_entity();
        let _ = system.set(ent, Position { x: 0.0, y: 0.0 });
        if i % N_POS_PER_VEL == 0 {
            let _ = system.set(ent, Velocity { dx: 0.0, dy: 0.0 });
        }
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
        let filter = component_filter!(Position, Velocity);
        system.collect_with(&filter, &mut ids);
        for id in ids {
            let velocity = system.get::<Velocity>(id).unwrap();
            let mut position = system.borrow_mut::<Position>(id).unwrap();
            position.x += velocity.dx;
            position.y += velocity.dy;
        }

        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position);
        system.collect_with(&filter, &mut ids);
        for id in ids {
            let _ = system.borrow::<Position>(id).unwrap();
        }
    });
}
