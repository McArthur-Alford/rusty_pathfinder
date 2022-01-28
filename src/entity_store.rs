use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
};

use hecs::{Component, Entity};

type EntityId = usize;

pub struct EntityStore {
    component_maps: Vec<Box<dyn ComponentContainer>>,
}

impl EntityStore {
    pub fn new() -> Self {
        Self {
            component_maps: Vec::new(),
        }
    }

    /// clear_entity
    /// removes all components associated with the given id.
    pub fn clear_entity(&mut self, entity: EntityId) {
        for component_map in self.component_maps.iter_mut() {
            component_map.set_none(entity);
        }
    }

    /// add_component
    /// adds a component of ComponentType to the map with an associated id.
    pub fn set_component<ComponentType: 'static>(
        &mut self,
        entity: EntityId,
        component: ComponentType,
    ) {
        // Check all component maps for a valid store and insert:
        for component_map in self.component_maps.iter_mut() {
            if let Some(component_map) = component_map
                .as_any_mut()
                .downcast_mut::<HashMap<EntityId, Option<ComponentType>>>()
            {
                component_map.insert(entity, Some(component));
                return;
            }
        }

        // The component map doesn't exist, so we create a new empty one:
        let mut new_component_map: HashMap<EntityId, Option<ComponentType>> = HashMap::new();

        // Set the component at our entity index in the vec:
        new_component_map.insert(entity, Some(component));

        // Put the new vec in the EntityStore:
        self.component_maps.push(Box::new(new_component_map));
    }

    pub fn clear_component<ComponentType: 'static>(&mut self, entity: EntityId) {
        for component_map in self.component_maps.iter_mut() {
            if let Some(component_map) = component_map
                .as_any_mut()
                .downcast_mut::<HashMap<EntityId, Option<ComponentType>>>()
            {
                component_map.insert(entity, None);
                return;
            }
        }
    }

    /// get_component
    /// returns an immutable reference to a component associated with a given id:
    pub fn get_component<ComponentType: 'static>(
        &mut self,
        entity: EntityId,
    ) -> Option<&ComponentType> {
        for component_map in self.component_maps.iter_mut() {
            if let Some(component_map) = component_map
                .as_any_mut()
                .downcast_mut::<HashMap<EntityId, Option<ComponentType>>>()
            {
                return match component_map.get(&entity) {
                    Some(component_map) => component_map.as_ref(),
                    None => None,
                };
            }
        }
        None
    }

    pub fn get_map<ComponentType: 'static>(
        &mut self,
    ) -> Option<&HashMap<EntityId, Option<ComponentType>>> {
        for component_map in self.component_maps.iter_mut() {
            if let Some(component_map) = component_map
                .as_any()
                .downcast_ref::<HashMap<EntityId, Option<ComponentType>>>()
            {
                return Some(component_map);
            }
        }

        return None;
    }

    pub fn commit(&mut self, action: Action) {
        self.component_maps.extend(action.insertions.component_maps);
        for component_set in action.removals.component_sets {
            for component_map in self.component_maps {
                let a = component_map.as_any();
                if a.type_id()
            }
        }

        let a = Vec::<Option<i32>>::new();
        let b = Vec::<None>::new();
        a.extend(b);
    }
}

struct EntitySet {
    component_sets: HashMap<TypeId, HashSet<EntityId>>,
}

impl EntitySet {
    pub fn new() -> Self {
        Self {
            component_sets: HashMap::new(),
        }
    }

    pub fn set_component<ComponentType: 'static>(&mut self, entity: EntityId) {
        for component_set in self.component_sets.iter_mut() {
            if component_set.0 == &TypeId::of::<ComponentType>() {
                component_set.1.insert(entity);
                return;
            }
        }

        // No set for this TypeId, create a new one:
        let mut new_component_set: HashSet<EntityId> = HashSet::new();
        new_component_set.insert(entity);

        self.component_sets
            .insert(TypeId::of::<ComponentType>(), new_component_set);
    }

    pub fn clear_component<ComponentType: 'static>(&mut self, entity: EntityId) {
        for component_set in self.component_sets.iter_mut() {
            if component_set.0 == &TypeId::of::<ComponentType>() {
                component_set.1.remove(&entity);
            }
        }
    }
}

pub trait ComponentContainer {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn set_none(&mut self, entity: EntityId);
}

impl<T: 'static> ComponentContainer for HashMap<EntityId, Option<T>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn set_none(&mut self, entity: EntityId) {
        self.remove(&entity);
    }
}

impl ComponentContainer for (HashSet<EntityId>, TypeId) {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn set_none(&mut self, entity: EntityId) {
        self.0.remove(&entity);
    }
}

struct Action {
    pub insertions: EntityStore,
    pub removals: EntitySet,
}
