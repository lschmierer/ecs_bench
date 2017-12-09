#![feature(test)]

extern crate test;
use test::Bencher;

extern crate specs;

extern crate ecs_bench;

use specs::{World, Entity, Component, Join, ReadStorage, RunNow, System, VecStorage, WriteStorage};

use ecs_bench::pos_vel::{Position, Velocity, N_POS_PER_VEL, N_POS};

struct PosComp(Position);
impl Component for PosComp {
    type Storage = VecStorage<PosComp>;
}

struct VelComp(Velocity);
impl Component for VelComp {
    type Storage = VecStorage<VelComp>;
}

struct VelSys;
impl<'a> System<'a> for VelSys {
    type SystemData = (ReadStorage<'a, VelComp>, WriteStorage<'a, PosComp>);
    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        (&mut pos, &vel).join().for_each(|(p, v)| {
            p.0.x += v.0.dx;
            p.0.y += v.0.dy;
        });
    }
}

fn build() -> (World, VelSys) {
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

    let sys = VelSys;
    (w, sys)
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let (world, mut sys) = build();

    b.iter(|| {
        sys.run_now(&world.res);
    });
}
