#![feature(test)]

extern crate test;
use test::Bencher;

extern crate parsec;

extern crate ecs_bench;

use parsec::{Entity, Component, Scheduler, Storage, VecStorage};

use ecs_bench::pos_vel::{Position, Velocity, N_POS_VEL, N_POS};

struct PosComp(Position);
impl Component for PosComp {
    type Storage = VecStorage<PosComp>;
}

struct VelComp(Velocity);
impl Component for VelComp {
    type Storage = VecStorage<VelComp>;
}

fn build() -> Scheduler {
    let mut scheduler = {
        let mut w = parsec::World::new();
        w.register::<PosComp>();
        w.register::<VelComp>();
        Scheduler::new(w, 4)
    };

    // setup entities
    {
        let ents: Vec<Entity> = (0..N_POS_VEL + N_POS)
                                    .map(|_| scheduler.add_entity().build())
                                    .collect();

        let mut positions = scheduler.get_world().write::<PosComp>();
        let mut velocities = scheduler.get_world().write::<VelComp>();

        ents.iter()
            .take(N_POS_VEL)
            .inspect(|e| positions.add(*e.clone(), PosComp(Position { x: 0.0, y: 0.0 })))
            .inspect(|e| velocities.add(*e.clone(), VelComp(Velocity { dx: 0.0, dy: 0.0 })))
            .collect::<Vec<&Entity>>();

        ents.iter()
            .skip(N_POS_VEL)
            .inspect(|e| positions.add(*e.clone(), PosComp(Position { x: 0.0, y: 0.0 })))
            .collect::<Vec<&Entity>>();
    }

    scheduler
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}


#[bench]
fn bench_update(b: &mut Bencher) {
    let mut scheduler = build();

    b.iter(|| {
        scheduler.run1w1r(|p: &mut PosComp, v: &VelComp| {
            p.0.x += v.0.dx;
            p.0.y += v.0.dy;
        });
        scheduler.run0w1r(|_: &PosComp| {
        });
    });
}
