#![feature(test)]

extern crate test;
use test::Bencher;

extern crate rayon;
extern crate specs;

extern crate ecs_bench;

use rayon::prelude::*;
use specs::{World, Entity, Component, Dispatcher, DispatcherBuilder, ParJoin, ReadStorage, System,
            VecStorage, WriteStorage};

use ecs_bench::parallel::{R, W1, W2, N};

struct RComp(R);
impl Component for RComp {
    type Storage = VecStorage<RComp>;
}

struct W1Comp(W1);
impl Component for W1Comp {
    type Storage = VecStorage<W1Comp>;
}

struct W2Comp(W2);
impl Component for W2Comp {
    type Storage = VecStorage<W2Comp>;
}

struct Sys1;
impl<'a> System<'a> for Sys1 {
    type SystemData = (WriteStorage<'a, W1Comp>, ReadStorage<'a, RComp>);
    fn run(&mut self, (mut w1s, rs): Self::SystemData) {
        (&mut w1s, &rs).par_join().for_each(|(w1, r)| {
            w1.0.x = r.0.x;
        });
    }
}

struct Sys2;
impl<'a> System<'a> for Sys2 {
    type SystemData = (WriteStorage<'a, W2Comp>, ReadStorage<'a, RComp>);
    fn run(&mut self, (mut w2s, rs): Self::SystemData) {
        (&mut w2s, &rs).par_join().for_each(|(w2, r)| {
            w2.0.x = r.0.x;
        });
    }
}

fn build() -> (World, Dispatcher<'static, 'static>) {
    let mut w = World::new();
    w.register::<RComp>();
    w.register::<W1Comp>();
    w.register::<W2Comp>();

    // setup entities
    {
        let ents: Vec<Entity> = w.create_iter().take(N).collect();

        let mut rs = w.write::<RComp>();
        let mut w1s = w.write::<W1Comp>();
        let mut w2s = w.write::<W2Comp>();

        for e in ents {
            rs.insert(e, RComp(R { x: 0.0 }));
            w1s.insert(e, W1Comp(W1 { x: 0.0 }));
            w2s.insert(e, W2Comp(W2 { x: 0.0 }));
        }
    }

    let dispatcher = DispatcherBuilder::new()
        .add(Sys1, "sys1", &[])
        .add(Sys2, "sys2", &[])
        .build();
    (w, dispatcher)
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let (mut world, mut dispatcher) = build();
    
    b.iter(|| {
        dispatcher.dispatch(&mut world.res);
    });
}
