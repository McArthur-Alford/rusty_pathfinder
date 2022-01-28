use std::collections::{HashMap, HashSet};

use hecs::Entity;

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
        for
    }
}

struct EntitySet {
    component_sets: Vec<Box<dyn ComponentContainer>>,
}

impl EntitySet {
    pub fn new() -> Self {
        Self {
            component_sets: Vec::new(),
        }
    }

    pub fn set_component<ComponentType: 'static>(&mut self, entity: EntityId) {
        // Check all component sets for a valid store and insert:
        for component_set in self.component_sets.iter_mut() {
            if let Some(component_set) = component_set
                .as_any_mut()
                .downcast_mut::<(HashSet<EntityId>, Option<ComponentType>)>()
            {
                component_set.0.insert(entity);
            }
        }

        // The component set doesn't exist so we create a new empty one:
        let mut new_component_set: (HashSet<EntityId>, Option<ComponentType>) =
            (HashSet::<EntityId>::new(), None);

        new_component_set.0.insert(entity);

        self.component_sets.push(Box::new(new_component_set));
    }

    pub fn clear_component<ComponentType: 'static>(&mut self, entity: EntityId) {
        for component_set in self.component_sets.iter_mut() {
            if let Some(component_set) = component_set
                .as_any_mut()
                .downcast_mut::<(HashSet<EntityId>, Option<ComponentType>)>()
            {
                component_set.0.remove(&entity);
            }
        }
    }

    pub fn get_set<ComponentType: 'static>(
        &mut self,
    ) -> Option<&(HashSet<EntityId>, Option<ComponentType>)> {
        for component_set in self.component_sets.iter_mut() {
            if let Some(component_set) = component_set
                .as_any()
                .downcast_ref::<(HashSet<EntityId>, Option<ComponentType>)>()
            {
                return Some(component_set);
            }
        }
        return None;
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

impl<T: 'static> ComponentContainer for (HashSet<EntityId>, Option<T>) {
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
