use std::{thread, time::Duration};

use document::Document;
use effects::{effect_system, EffectStorage, FastHealing};
use hecs::*;
use time::{EffectDuration, WorldTime};

use crate::data_character::Health;

mod data_character;
mod document;
mod effects;
mod time;

fn main() {
    let mut world = World::new();

    let mut world_time = WorldTime::default();


    // Spawning the player:
    let mut player = Document::new();
    let mut player_effects = EffectStorage::new();
    player_effects
        .effects
        .push((Box::new(FastHealing(4)), EffectDuration::Seconds {start: world_time.passed, duration: Duration::from_secs(6)}));
    player.0.spawn((Health::default(),));
    world.spawn((player, player_effects));


    loop {
        println!("{:?}", world_time.passed);

        effect_system(&mut world, &world_time);

        for (_id, (document,)) in &mut world.query::<(&mut Document,)>() {
            for (_id, (health,)) in &mut document.0.query::<(&mut Health,)>() {
                println!("Health Remaining: {:?}", health.current_health);
            }
        }

        world_time.real_time_tick();
        if world_time.passed > Duration::from_secs(6) {
            break;
        }

        thread::sleep(Duration::from_millis(300));
    }
}
