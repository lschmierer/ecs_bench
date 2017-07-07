#![feature(test)]

extern crate test;
use test::Bencher;

extern crate specs;

extern crate ecs_bench;

use specs::{World, Entity, Component, DenseVecStorage, Dispatcher, DispatcherBuilder, Join, ReadStorage, System, WriteStorage};

use ecs_bench::pos_vel::{Position, Velocity, N_POS_PER_VEL, N_POS};

struct PosComp(Position);
impl Component for PosComp {
    type Storage = DenseVecStorage<PosComp>;
}

struct VelComp(Velocity);
impl Component for VelComp {
    type Storage = DenseVecStorage<VelComp>;
}

struct VelSys;
impl<'a> System<'a> for VelSys {
    type SystemData = (ReadStorage<'a, VelComp>, WriteStorage<'a, PosComp>);
    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (p, v) in (&mut pos, &vel).join() {
            p.0.x += v.0.dx;
            p.0.y += v.0.dy;
        }
    }
}

struct PosSys;
impl<'a> System<'a> for PosSys {
    type SystemData = (ReadStorage<'a, PosComp>);
    fn run(&mut self, pos: Self::SystemData) { }
}

fn build<'a, 'b>() -> (World, Dispatcher<'a, 'b>) {
    let mut w = World::new();
    w.register::<PosComp>();
    w.register::<VelComp>();

    // setup entities
    {
        let ents: Vec<Entity> = w.create_iter().take(N_POS).collect();

        let mut positions = w.write::<PosComp>();
        let mut velocities = w.write::<VelComp>();

        for (i, e) in ents.iter().enumerate() {
            positions.insert(*e, PosComp(Position { x: 0.0, y: 0.0 }));
            if i % N_POS_PER_VEL == 0 {
                velocities.insert(*e, VelComp(Velocity { dx: 0.0, dy: 0.0 }));
            }
        }
    }

    let dispatcher = DispatcherBuilder::new()
        .add(VelSys, "vel", &[])
        .add(PosSys, "pos", &[])
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
