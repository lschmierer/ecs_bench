#![feature(test)]

extern crate test;
use test::Bencher;

extern crate parsec;

extern crate ecs_bench;

use ecs_bench::pos_vel::{Position, Velocity, N_POS_VEL, N_POS};

struct PosComp(Position);
impl parsec::Component for PosComp {
    type Storage = parsec::VecStorage<PosComp>;
}

struct VelComp(Velocity);
impl parsec::Component for VelComp {
    type Storage = parsec::VecStorage<VelComp>;
}

fn build() -> parsec::Scheduler {
    let mut scheduler = {
        let mut w = parsec::World::new();
        w.register::<PosComp>();
        w.register::<VelComp>();
        parsec::Scheduler::new(w, 4)
    };

    // setup entities
    for _ in 0..N_POS_VEL {
        scheduler.add_entity()
                 .with(PosComp(Position { x: 0.0, y: 0.0 }))
                 .with(VelComp(Velocity { dx: 0.0, dy: 0.0 }))
                 .build();
    }
    for _ in 0..N_POS {
        scheduler.add_entity()
                 .with(PosComp(Position { x: 0.0, y: 0.0 }))
                 .build();
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
