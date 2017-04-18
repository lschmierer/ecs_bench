#![feature(test)]

extern crate test;
use test::Bencher;

extern crate constellation;

extern crate ecs_bench;

use constellation::{SystemCommandBuffer, VecResource, World};

use ecs_bench::parallel::{R, W1, W2, N};

type Rs = VecResource<R>;
type W1s = VecResource<W1>;
type W2s = VecResource<W2>;

fn build() -> World {
    let mut update = SystemCommandBuffer::default();

    // setup entities
    update.queue_systems(|scope| {
        scope.run_r0w3(|ctx, rs: &mut Rs, w1s: &mut W1s, w2s: &mut W2s| {
            for _ in 0..N {
                let e = ctx.create();
                rs.add(e, R { x: 0.0 });
                w1s.add(e, W1 { x: 0.0 });
                w2s.add(e, W2 { x: 0.0 });
            }
        });
    });

    let mut w = World::new();
    w.register_resource(Rs::new());
    w.register_resource(W1s::new());
    w.register_resource(W2s::new());
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
    let mut update = SystemCommandBuffer::default();

    update.queue_systems(|scope| {
        scope.run_r1w1(|ctx, rs: &Rs, w1s: &mut W1s| {
            ctx.iter_r1w1(rs, w1s).components(|_, r, w1| w1.x = r.x);
        });
        scope.run_r1w1(|ctx, rs: &Rs, w2s: &mut W2s| {
            ctx.iter_r1w1(rs, w2s).components(|_, r, w2| w2.x = r.x);
        });
    });

    b.iter(|| {
        world.run(&mut update);
    });
}
