use specs::{rayon::vec, Component, VecStorage};
use std::sync;

pub trait Data {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

default impl<S: 'static> Data for S {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

pub struct DataSheetComponent {
    pub data: Vec<Box<dyn Data + Send + Sync>>,
}

impl DataSheetComponent {
    pub fn new() -> DataSheetComponent {
        DataSheetComponent { data: Vec::new() }
    }

    pub fn with(mut self, data: Box<dyn Data + Send + Sync>) -> Self {
        self.data.push(data);
        self
    }

    pub fn query<T: 'static>(&self) -> impl Iterator<Item = &T> {
        self.data
            .iter()
            .filter_map(|x| x.as_any().downcast_ref::<T>())
    }

    pub fn query_mut<T: 'static>(&mut self) -> impl Iterator<Item = &mut T> {
        self.data
            .iter_mut()
            .filter_map(|x| x.as_any_mut().downcast_mut::<T>())
    }
}

impl Component for DataSheetComponent {
    type Storage = VecStorage<Self>;
}

// Literally all the data components in mostly alphabetical order:

#[derive(Debug)]
pub struct Health(pub i32);

impl Health {
    fn default() -> Health {
        Health(0)
    }
}
impl Data for Health {}

// Ability Scores

pub struct Strength {
    base_score: i32,
    effective_score: i32,
    modifier: i32,
}

impl Strength {
    fn default() -> Strength {
        Strength {
            base_score: 0,
            effective_score: 0,
            modifier: 0,
        }
    }
}
impl Data for Strength {}

pub struct Dexterity {
    base_score: i32,
    effective_score: i32,
    modifier: i32,
}

impl Dexterity {
    fn default() -> Dexterity {
        Dexterity {
            base_score: 0,
            effective_score: 0,
            modifier: 0,
        }
    }
}
impl Data for Dexterity {}

pub struct Constitution {
    base_score: i32,
    effective_score: i32,
    modifier: i32,
}

impl Constitution {
    fn default() -> Constitution {
        Constitution {
            base_score: 0,
            effective_score: 0,
            modifier: 0,
        }
    }
}
impl Data for Constitution {}

pub struct Wisdom {
    base_score: i32,
    effective_score: i32,
    modifier: i32,
}

impl Wisdom {
    fn default() -> Wisdom {
        Wisdom {
            base_score: 0,
            effective_score: 0,
            modifier: 0,
        }
    }
}
impl Data for Wisdom {}

pub struct Intelligence {
    base_score: i32,
    effective_score: i32,
    modifier: i32,
}

impl Intelligence {
    fn default() -> Intelligence {
        Intelligence {
            base_score: 0,
            effective_score: 0,
            modifier: 0,
        }
    }
}
impl Data for Intelligence {}

pub struct Charisma {
    base_score: i32,
    effective_score: i32,
    modifier: i32,
}

impl Charisma {
    fn default() -> Charisma {
        Charisma {
            base_score: 0,
            effective_score: 0,
            modifier: 0,
        }
    }
}
impl Data for Charisma {}
