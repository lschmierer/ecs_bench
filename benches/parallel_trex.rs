#![feature(test)]
extern crate test;
use test::Bencher;

#[macro_use]
extern crate trex;

use trex::{System, EventQueue, EventEmitter, Simulation, World, ComponentFilter};

extern crate ecs_bench;
use ecs_bench::parallel::{R, W1, W2, N};

pub struct RComp(R);
pub struct W1Comp(W1);
pub struct W2Comp(W2);
components!(RComp, W1Comp, W2Comp);

pub struct W1System {
    filter: ComponentFilter,
}

impl W1System {
    pub fn new() -> W1System {
        W1System {
            filter: ComponentFilter::new()
                        .with::<RComp>()
                        .with::<W1Comp>(),
        }
    }
}

impl System for W1System {
    fn update(&mut self,
              world: &mut World,
              _queue: &EventQueue,
              _emitter: &mut EventEmitter,
              _dt: f32) {
        for entity in world.filter(&self.filter) {
            let &RComp(R { x }) = world.get(entity).unwrap();
            let mut w1 = world.get_mut::<W1Comp>(entity).unwrap();
            w1.0.x = x;
        }
    }
}

pub struct W2System {
    filter: ComponentFilter,
}

impl W2System {
    pub fn new() -> W2System {
        W2System {
            filter: ComponentFilter::new()
                        .with::<RComp>()
                        .with::<W2Comp>(),
        }
    }
}

impl System for W2System {
    fn update(&mut self,
              world: &mut World,
              _queue: &EventQueue,
              _emitter: &mut EventEmitter,
              _dt: f32) {
        for entity in world.filter(&self.filter) {
            let &RComp(R { x }) = world.get(entity).unwrap();
            let mut w2 = world.get_mut::<W2Comp>(entity).unwrap();
            w2.0.x = x;
        }
    }
}

fn build() -> Simulation {
    let world = {
        let mut world = World::new();
        world.register::<RComp>();
        world.register::<W1Comp>();
        world.register::<W2Comp>();

        for _ in 0..N {
            let entity = world.create();
            world.add(entity, RComp(R { x: 0.0 }));
            world.add(entity, W1Comp(W1 { x: 0.0 }));
            world.add(entity, W2Comp(W2 { x: 0.0 }));
        }

        world
    };

    let queue = EventQueue::new();
    let emitter = EventEmitter::new();

    let mut simulation = Simulation::new(world, queue, emitter);
    simulation.register(W1System::new());
    simulation.register(W2System::new());
    simulation
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut simulation = build();

    b.iter(|| {
        simulation.update(1.0);
    });
}
