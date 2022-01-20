#![feature(specialization)]

use crate::data::*;
use crate::dice::*;
use crate::effects::*;
use crate::time::*;
use specs::prelude::*;
use std::time::*;

mod data;
mod dice;
mod effects;
mod time;

fn main() {
    let mut world = World::new();
    world.register::<DataSheetComponent>();
    world.register::<EffectComponent>();

    world.insert(WorldTime::default());

    world
        .create_entity()
        .with(DataSheetComponent::new().with(Box::new(Health(5))))
        .with(EffectComponent::new().with(
            Box::new(FastHealing(5)),
            EffectDuration::Seconds {
                start: std::time::Duration::new(0, 0),
                duration: std::time::Duration::from_secs(3),
            },
        ))
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(EffectSystem, "EffectSystem", &[])
        .with(DebugSystem, "DebugSystem", &[])
        .build();

    for i in 0..100 {
        dispatcher.dispatch(&mut world);

        std::thread::sleep(Duration::from_millis(100));

        println!("{:?}", world.fetch_mut::<WorldTime>().passed);
        world.fetch_mut::<WorldTime>().real_time_tick();
    }
}

pub struct DebugSystem;

impl<'a> System<'a> for DebugSystem {
    type SystemData = ReadStorage<'a, DataSheetComponent>;

    fn run(&mut self, (datasheet): Self::SystemData) {
        println!("Debugging Actors:");

        for datasheet in (&datasheet).join() {
            datasheet
                .query::<Health>()
                .for_each(|x| println!("Actor {:p} has {} health", &datasheet, x.0));
        }
    }
}
