#![feature(test)]

extern crate test;
use test::Bencher;

extern crate constellation;

extern crate ecs_bench;

use constellation::{SystemCommandBuffer, VecResource, World};

use ecs_bench::pos_vel::{Position, Velocity, N_POS_VEL, N_POS};

type Positions = VecResource<Position>;
type Velocities = VecResource<Velocity>;

fn build() -> World {
    // setup entities
    let mut update = SystemCommandBuffer::default();
    update.queue_systems(|scope| {
        scope.run_r0w2(|ctx, positions: &mut Positions, velocities: &mut Velocities| {
            for _ in 0..N_POS_VEL {
                let e = ctx.create();
                positions.add(e, Position { x: 0.0, y: 0.0 });
                velocities.add(e, Velocity { dx: 0.0, dy: 0.0 });
            }
        });

        scope.run_r0w1(|ctx, positions: &mut Positions| {
            for _ in 0..N_POS {
                let e = ctx.create();
                positions.add(e, Position { x: 0.0, y: 0.0 });
            }
        });
    });

    let mut w = World::new();
    w.register_resource(Positions::new());
    w.register_resource(Velocities::new());
    w.run(&mut update);
    w
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut world = build();

    b.iter(|| {
        let mut update = SystemCommandBuffer::default();
        update.queue_systems(|scope| {
            scope.run_r1w1(|ctx, velocities: &Velocities, positions: &mut Positions| {
                ctx.iter_r1w1(velocities, positions).components(|_, v, p| {
                    p.x += v.dx;
                    p.y += v.dy;
                });
            });
        });
        world.run(&mut update);
    });
}
