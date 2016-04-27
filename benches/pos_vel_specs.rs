#![feature(test)]

extern crate test;
use test::Bencher;

extern crate specs;

extern crate ecs_bench;

use specs::{World, Entity, Component, Planner, VecStorage};

use ecs_bench::pos_vel::{Position, Velocity, N_POS_VEL, N_POS};

struct PosComp(Position);
impl Component for PosComp {
    type Storage = VecStorage<PosComp>;
}

struct VelComp(Velocity);
impl Component for VelComp {
    type Storage = VecStorage<VelComp>;
}

fn build() -> Planner<()> {
    let mut w = World::new();
    w.register::<PosComp>();
    w.register::<VelComp>();

    // setup entities
    {
        let ents: Vec<Entity> = w.create_iter().take(N_POS_VEL + N_POS).collect();

        let mut positions = w.write::<PosComp>();
        let mut velocities = w.write::<VelComp>();

        for e in ents[..N_POS_VEL].iter() {
            positions.insert(*e, PosComp(Position { x: 0.0, y: 0.0 }));
            velocities.insert(*e, VelComp(Velocity { dx: 0.0, dy: 0.0 }));
        }
        for e in ents[N_POS_VEL..].iter() {
            positions.insert(*e, PosComp(Position { x: 0.0, y: 0.0 }));
        }
    }

    Planner::new(w, 4)
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut planner = build();

    b.iter(|| {
        planner.run1w1r(|p: &mut PosComp, v: &VelComp| {
            p.0.x += v.0.dx;
            p.0.y += v.0.dy;
        });
        planner.run0w1r(|_: &PosComp| {
        });
        planner.wait();
    });
}
