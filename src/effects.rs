use hecs::World;

use crate::{document::Document, time::{EffectDuration, WorldTime}, data_character::Health};

// EffectComponent Implementation:

pub struct EffectStorage {
    pub effects: Vec<(Box<dyn Effect + Send + Sync>, EffectDuration)>,
}

impl EffectStorage {
    pub fn new() -> EffectStorage {
        EffectStorage {
            effects: Vec::new(),
        }
    }

    pub fn with(mut self, effect: Box<dyn Effect + Send + Sync>, duration: EffectDuration) -> Self {
        self.effects.push((effect, duration));
        self
    }
}

// Effect Staging:

pub enum EffectStage {
    BaseValues, // These overwrite values, disregarding any data that comes before.
    AdditiveModifiers,
    MultiplicativeModifiers,
    Healing,
}

static STAGE_ORDER: &'static [EffectStage] = &[
    EffectStage::BaseValues,
    EffectStage::AdditiveModifiers,
    EffectStage::MultiplicativeModifiers,
    EffectStage::Healing,
];

pub fn effect_system(world: &mut World, time: &WorldTime) {
    for (_id, (document, effects)) in world.query_mut::<(&mut Document, &mut EffectStorage)>() {
        let mut stage = 0;
        while stage < STAGE_ORDER.len() {
            effects.effects.iter_mut().for_each(|x| {
                match x.1 {
                    EffectDuration::Permenant => (),
                    EffectDuration::Seconds { start, duration } => {
                        if start + duration < time.passed {
                            drop(x);
                            return;
                        }
                    }
                }
                if x.0.ready(&STAGE_ORDER[stage]) {
                    x.0.update(document);
                }
            });
            stage += 1;
        }
    }
}

// // Effect Implementation:

pub trait Effect {
    fn reset(&self) {}
    fn ready(&self, trigger: &EffectStage) -> bool;
    fn update(&self, datasheet: &mut Document);
}

// // -------- effects ---------

pub struct FastHealing(pub i32);

impl Effect for FastHealing {
    fn ready(&self, trigger: &EffectStage) -> bool {
        matches!(trigger, EffectStage::Healing)
    }

    fn update(&self, document: &mut Document) {
        for (_id, (health,)) in document.0.query_mut::<(&mut Health,)>() {
            health.current_health += self.0;
        }
    }
}

// pub struct CalculateHealth;

// impl Effect for CalculateHealth {
//     fn ready(&self, trigger: &EffectStage) -> bool {
//         matches!(trigger, EffectStage::BaseValues)
//     }

//     fn update(&self, datasheet: &mut DataSheetComponent) {
//         // let query2 = datasheet.query::<Constitution>().borrow();
//         // let query = datasheet.query_mut::<Health>().borrow_mut();
//         // query.zip(query2).for_each(|(health, con)| {
//         //     health.max_health = con.0.effective_score as u32 * 2;
//         // });
//     }
// }
