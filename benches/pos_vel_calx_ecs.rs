#![feature(test)]
extern crate test;
use test::Bencher;

#[macro_use]
extern crate calx_ecs;
extern crate rustc_serialize;

extern crate ecs_bench;

use calx_ecs::Entity;

use ecs_bench::pos_vel::{Position, Velocity, N_POS_VEL, N_POS};

Ecs! {
    pos: Position,
    vel: Velocity,
}

fn build() -> Ecs {
    let mut ecs = Ecs::new();

    // setup entities
    for _ in 0..N_POS_VEL {
        let e = ecs.make();
        ecs.pos.insert(e, Position { x: 0.0, y: 0.0 });
        ecs.vel.insert(e, Velocity { dx: 0.0, dy: 0.0 });
    }
    for _ in 0..N_POS {
        let e = ecs.make();
        ecs.pos.insert(e, Position { x: 0.0, y: 0.0 });
    }

    ecs
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut ecs = build();

    b.iter(|| {
        // Update
        let with_velocity: Vec<Entity> = ecs.vel.iter().map(|(&e, _)| e).collect();
        for &e in &with_velocity {
            let vel = ecs.vel[e];
            ecs.pos.get_mut(e).map(|pos| {
                pos.x += vel.dx;
                pos.y += vel.dy;
            });
        }

        // Render
        for &e in ecs.iter() {
            ecs.pos.get(e).map(|_pos| {});
        }
    });
}
