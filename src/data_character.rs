// use specs::{EntityBuilder, storage::StorageEntry};

// use crate::data::{DataSheetComponent};
// use specs::prelude::*;

// pub fn new_character() -> DataSheetComponent {
//     DataSheetComponent::new()
// }

// Health:

#[derive(Debug)]
pub struct Health {
    pub current_health: i32,
    pub max_health: u32,
    pub temporary: u32
}

impl Default for Health {
    fn default() -> Health {
        Health {
            current_health: 0,
            max_health: 0,
            temporary: 0
        }
    }

}

impl Health {
    fn new(current_health: i32, max_health: u32) -> Health {
        Health { current_health, max_health, temporary: 0 }
    }
}


// // Ability Scores:

// pub struct AbilityScore {
//     pub base_score: i32,
//     pub effective_score: i32,
//     pub modifier: i32,
// }
// impl Default for AbilityScore {
//     fn default() -> AbilityScore {
//         AbilityScore {
//             base_score: 0,
//             effective_score: 0,
//             modifier: 0,
//         }
//     }
// }

// #[derive(Default)]
// pub struct Strength(pub AbilityScore);

// #[derive(Default)]
// pub struct Dexterity(pub AbilityScore);

// #[derive(Default)]
// pub struct Constitution(pub AbilityScore);

// #[derive(Default)]
// pub struct Intelligence(pub AbilityScore);

// #[derive(Default)]
// pub struct Charisma(pub AbilityScore);

// #[derive(Default)]
// pub struct Wisdom(pub AbilityScore);
