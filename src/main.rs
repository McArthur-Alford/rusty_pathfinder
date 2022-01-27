use std::{collections::{HashMap, HashSet}, default};

type EntityId = usize;

struct EntityStore {
    position: HashMap<EntityId, (u32, u32)>,
    solid: HashSet<EntityId>,
    // Add new components as hashsets/maps ..
}

impl EntityStore {
    fn new() -> EntityStore {
        return EntityStore {
            position: HashMap::default(),
            solid: HashSet::default()
            // Repeat for each component ...
        }
    }

    fn get_position(&self, id: EntityId) -> Option<(u32, u32)> {
        self.position.get(&id).map(|v| *v)
    }
    // Repeat for each data component ...

    fn contains_solid(&self, id: EntityId) -> bool {
        self.solid.contains(&id)
    }
    // Repeat for each flag component ...
}


struct RemovedComponents {
    position: HashSet<EntityId>,
    solid: HashSet<EntityId>,
}

struct Action {
    additions: EntityStore,
    removals: RemovedComponents
}

impl Action {
    pub fn remove_position(&mut self, id: EntityId) {
        self.removals.position.insert(id);
    }

    pub fn remove_solid(&mut self, id: EntityId) {
        self.removals.solid.insert(id);
    }

    pub fn insert_position(&mut self, id: EntityId, value: (u32, u32)) {
        self.additions.position.insert(id, value);
    }

    pub fn insert_solid(&mut self, id: EntityId) {
        self.additions.solid.insert(id);
    }
}

fn commit_action(state: &mut EntityStore, action: &mut Action) {
    for id in action.removals.position.drain() {
        state.position.remove(&id);
    }

    for id in action.removals.solid.drain() {
        state.solid.remove(&id);
    }

    // Repeated for each component type ...

    for (id, value) in action.additions.position.drain() {
        state.position.insert(id, value);
    }

    for id in action.additions.solid.drain() {
        state.solid.insert(id);
    }
}

struct Game {
    entity_store: EntityStore,
}

impl Game {
    fn game_loop(&mut self) {}

    fn process_actions(&mut self) {}
}

fn main() {
    let mut game_state = Game {
        entity_store: EntityStore::new(),
    };
    loop {
        game_state.game_loop();
    }
}

// use std::{thread, time::Duration};

// use document::Document;
// use effects::{effect_system, EffectStorage, FastHealing};
// use hecs::*;
// use time::{EffectDuration, WorldTime};

// use crate::data_character::Health;

// mod data_character;
// mod document;
// mod effects;
// mod time;
// mod action;

// fn main() {
//     let mut world = World::new();

//     let mut world_time = WorldTime::default();

//     // Spawning the player:
//     let mut player = Document::new();
//     let mut player_effects = EffectStorage::new();
//     player_effects
//         .effects
//         .push((Box::new(FastHealing(4)), EffectDuration::Seconds {start: world_time.passed, duration: Duration::from_secs(6)}));
//     player.world.spawn((Health::default(),));
//     world.spawn((player, player_effects));

//     loop {
//         println!("{:?}", world_time.passed);

//         effect_system(&mut world, &world_time);

//         for (_id, (document,)) in &mut world.query::<(&mut Document,)>() {
//             for (_id, (health,)) in &mut document.world.query::<(&mut Health,)>() {
//                 println!("Health Remaining: {:?}", health.current_health);
//             }
//         }

//         world_time.real_time_tick();
//         if world_time.passed > Duration::from_secs(6) {
//             break;
//         }

//         thread::sleep(Duration::from_millis(300));
//     }
// }
