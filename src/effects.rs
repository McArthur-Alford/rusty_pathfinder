use std::time::Duration;

use crate::{data::*, time::{EffectDuration, WorldTime}};
use specs::{Component, Join, System, VecStorage, WriteStorage, shred::Resource, WorldExt, Read};

// EffectComponent Implementation:

pub struct EffectComponent {
    pub effects: Vec<(Box<dyn Effect + Send + Sync>, EffectDuration)>,
}

impl EffectComponent {
    pub fn new() -> EffectComponent {
        EffectComponent {
            effects: Vec::new(),
        }
    }

    pub fn with(mut self, effect: Box<dyn Effect + Send + Sync>, duration: EffectDuration) -> Self {
        self.effects.push((effect, duration));
        self
    }
}

impl Component for EffectComponent {
    type Storage = VecStorage<Self>;
}

// Effect Staging:

pub enum EffectStage {
    BaseValues,
    AdditiveModifiers,
    MultiplicativeModifiers,
    Healing,
}

static StageOrder: &'static [EffectStage] = &[
    EffectStage::BaseValues,
    EffectStage::AdditiveModifiers,
    EffectStage::MultiplicativeModifiers,
    EffectStage::Healing,
];

pub struct EffectSystem;

impl<'a> System<'a> for EffectSystem {
    type SystemData = (
        WriteStorage<'a, DataSheetComponent>,
        WriteStorage<'a, EffectComponent>,
        Read<'a, WorldTime>
    );

    fn run(&mut self, (mut datasheet, mut effects, time): Self::SystemData) {
        for (datasheet, effects) in (&mut datasheet, &mut effects).join() {
            let mut stage = 0;
            while stage < StageOrder.len() {
                effects.effects.iter_mut().for_each(|x| {
                    match x.1 {
                        EffectDuration::Permenant => (),
                        EffectDuration::Seconds { start, duration } => {
                            if start + duration < time.passed {
                                drop(x); return;
                            }
                        } 
                    }
                    if x.0.ready(&StageOrder[stage]) {
                        x.0.update(datasheet);
                    }
                });
                stage += 1;
            }
        }
    }
}

// Effect Implementation:

pub trait Effect {
    fn reset(&self) {}
    fn ready(&self, trigger: &EffectStage) -> bool;
    fn update(&self, datasheet: &mut DataSheetComponent);
}

// -------- effects ---------

pub struct FastHealing(pub i32);

impl Effect for FastHealing {
    fn ready(&self, trigger: &EffectStage) -> bool {
        matches!(trigger, EffectStage::Healing)
    }

    fn update(&self, datasheet: &mut DataSheetComponent) {
        let query = datasheet.query_mut::<Health>();
        query.for_each(|x| {
            x.0 += self.0;
        });
    }
}
