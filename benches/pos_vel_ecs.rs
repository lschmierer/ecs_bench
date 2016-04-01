#![feature(test)]

extern crate test;
use test::Bencher;

#[macro_use]
extern crate ecs;

extern crate ecs_bench;

use ecs::system::{EntityProcess, EntitySystem, System};
use ecs::{DataHelper, EntityIter, World};
use ecs::BuildData;

use ecs_bench::pos_vel::{Position, Velocity, N_POS_VEL, N_POS};

components! {
    struct MyComponents {
        #[hot] position: Position,
        #[cold] velocity: Velocity,
    }
}

pub struct Physics;

impl System for Physics {
    type Components = MyComponents;
    type Services = ();
}

impl EntityProcess for Physics {
    fn process(&mut self,
               entities: EntityIter<MyComponents>,
               data: &mut DataHelper<MyComponents, ()>) {
        for e in entities {
            let mut position = data.position[e];
            let velocity = data.velocity[e];
            position.x += velocity.dx;
            position.y += velocity.dy;
            data.position[e] = position;
        }
    }
}

pub struct Render;

impl System for Render {
    type Components = MyComponents;
    type Services = ();
}

impl EntityProcess for Render {
    fn process(&mut self,
               entities: EntityIter<MyComponents>,
               _: &mut DataHelper<MyComponents, ()>) {
        for _ in entities {
        }
    }
}

systems! {
    struct MySystems<MyComponents, ()> {
        active: {
            physics: EntitySystem<Physics> = EntitySystem::new(
                Physics,
                aspect!(<MyComponents> all: [position, velocity])),
            render: EntitySystem<Render> = EntitySystem::new(
                Render,
                aspect!(<MyComponents> all: [position])),
        },
        passive: {
        }
    }
}

fn build() -> World<MySystems> {
    let mut world = World::<MySystems>::new();

    // setup entities
    for _ in 0..N_POS_VEL {
        let _ = world.create_entity(|entity: BuildData<MyComponents>,
                                     data: &mut MyComponents| {
            data.position.add(&entity, Position { x: 0.0, y: 0.0 });
            data.velocity.add(&entity, Velocity { dx: 0.0, dy: 0.0 });
        });
    }
    for _ in 0..N_POS {
        let _ = world.create_entity(|entity: BuildData<MyComponents>,
                                     data: &mut MyComponents| {
            data.position.add(&entity, Position { x: 0.0, y: 0.0 });
        });
    }

    world
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| {
        build()
    });
}


#[bench]
fn bench_update(b: &mut Bencher) {
    let mut world = build();

    b.iter(|| {
        world.update();
    });
}
