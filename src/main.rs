#![feature(specialization)]

use crate::data::*;
use crate::dice::*;
use crate::effects::*;
use specs::prelude::*;
use std::time::*;

mod data;
mod dice;
mod effects;

fn main() {
    let mut world = World::new();
    world.register::<DataSheetComponent>();
    world.register::<EffectComponent>();

    world
        .create_entity()
        .with(DataSheetComponent::new().with(Box::new(Health(5))))
        .with(EffectComponent::new().with(Box::new(FastHealing(5))))
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(EffectSystem, "EffectSystem", &[])
        .with(DebugSystem, "DebugSystem", &[])
        .build();

    dispatcher.setup(&mut world);

    for i in 0..5 {
        let tick_time = Instant::now();
        println!("\nGame Tick: {}", i);
        dispatcher.dispatch(&mut world);
        let duration = tick_time.elapsed();
        println!("Tick Lasted {} microseconds", duration.as_micros());
    }

    println!("");

    let dm = DiceManager;
    println!(
        "Range Test: {} \nD(20) test {}, \nPercentile test {}\n",
        dm.roll(Dice::Range(0..2)),
        dm.roll(Dice::D(20)),
        dm.roll(Dice::Percentile)
    );

    println!("DC10 flat check result: {:?}", dm.check(Dice::D(20), 10));
    println!("DC10 flat check result: {:?}", dm.check(Dice::D(20), 10));
    println!("DC5 flat check result: {:?}", dm.check(Dice::D(20), 5));
    println!("DC5 flat check result: {:?}", dm.check(Dice::D(20), 5));
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
