#![feature(test)]

extern crate test;
use test::Bencher;

#[macro_use]
extern crate ecs;

extern crate ecs_bench;

use ecs::system::{EntityProcess, EntitySystem, System};
use ecs::{DataHelper, EntityIter, World};
use ecs::BuildData;

use ecs_bench::parallel::{R, W1, W2, N};

components! {
    struct MyComponents {
        #[hot] r: R,
        #[hot] w1: W1,
        #[hot] w2: W2,
    }
}

pub struct W1System;

impl System for W1System {
    type Components = MyComponents;
    type Services = ();
}

impl EntityProcess for W1System {
    fn process(&mut self,
               entities: EntityIter<MyComponents>,
               data: &mut DataHelper<MyComponents, ()>) {
        for e in entities {
            let mut w1 = data.w1[e];
            let r = data.r[e];
            w1.x += r.x;
            data.w1[e] = w1;
        }
    }
}

pub struct W2System;

impl System for W2System {
    type Components = MyComponents;
    type Services = ();
}

impl EntityProcess for W2System {
    fn process(&mut self,
               entities: EntityIter<MyComponents>,
               data: &mut DataHelper<MyComponents, ()>) {
        for e in entities {
            let mut w2 = data.w2[e];
            let r = data.r[e];
            w2.x *= r.x;
            data.w2[e] = w2;
        }
    }
}

systems! {
    struct MySystems<MyComponents, ()> {
        active: {
            w1: EntitySystem<W1System> = EntitySystem::new(
                W1System,
                aspect!(<MyComponents> all: [r, w1])),
            w2: EntitySystem<W2System> = EntitySystem::new(
                W2System,
                aspect!(<MyComponents> all: [r, w2])),
        },
        passive: {
        }
    }
}

fn build() -> World<MySystems> {
    let mut world = World::<MySystems>::new();

    // setup entities
    for _ in 0..N {
        let _ = world.create_entity(|entity: BuildData<MyComponents>, data: &mut MyComponents| {
            data.r.add(&entity, R { x: 0.0 });
            data.w1.add(&entity, W1 { x: 0.0 });
            data.w2.add(&entity, W2 { x: 0.0 });
        });
    }

    world
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut world = build();

    b.iter(|| {
        world.update();
    });
}
