use std::{
    collections::HashMap,
};

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
    pub fn set_entity_none(&mut self, entity: EntityId) {
        for component_map in self.component_maps.iter_mut() {
            component_map.set_none(entity);
        }
    }

    pub fn clear_entity(&mut self, entity: EntityId) {
        for component_map in self.component_maps.iter_mut() {}
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

    pub fn set_none<ComponentType: 'static>(&mut self, entity: EntityId) {
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
        &self,
        entity: EntityId,
    ) -> Option<&ComponentType> {
        for component_map in self.component_maps.iter() {
            if let Some(component_map) = component_map
                .as_any()
                .downcast_ref::<HashMap<EntityId, Option<ComponentType>>>()
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
        &self,
    ) -> Option<&HashMap<EntityId, Option<ComponentType>>> {
        for component_map in self.component_maps.iter() {
            if let Some(component_map) = component_map
                .as_any()
                .downcast_ref::<HashMap<EntityId, Option<ComponentType>>>()
            {
                return Some(component_map);
            }
        }

        return None;
    }

    pub fn get_map_mut<ComponentType: 'static>(
        &mut self,
    ) -> Option<&mut HashMap<EntityId, Option<ComponentType>>> {
        for component_map in self.component_maps.iter_mut() {
            if let Some(component_map) = component_map
                .as_any_mut()
                .downcast_mut::<HashMap<EntityId, Option<ComponentType>>>()
            {
                return Some(component_map);
            }
        }

        return None;
    }

    pub fn commit(&mut self, action: &mut Action) {
        for component_map in self.component_maps.iter_mut() {
            for action_map in action.insertions.component_maps.iter_mut() {
                component_map.commit(action_map);
            }
        }
    }
}

pub fn get_future_component<'a, ComponentType: 'static> (
    entity: EntityId,
    entity_store: &'a mut EntityStore,
    action: &'a mut Action
) -> Option<&'a ComponentType> {
    return match action.insertions.get_component::<ComponentType>(entity) {
        Some(thing) => Some(thing),
        None => entity_store.get_component::<ComponentType>(entity),
    }
}

pub trait ComponentContainer {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn set_none(&mut self, entity: EntityId);
    fn commit(&mut self, source: &mut Box<dyn ComponentContainer>);
}

impl<T: 'static> ComponentContainer for HashMap<EntityId, Option<T>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn set_none(&mut self, entity: EntityId) {
        self.insert(entity, None);
    }

    fn commit(&mut self, source: &mut Box<dyn ComponentContainer>) {
        if let Some(source) = source.as_any_mut().downcast_mut::<Self>() {
            self.extend(source.drain());
        }
    }
}

pub struct Action {
    pub insertions: EntityStore,
}

impl Action {
    pub fn new() -> Self {
        Self {
            insertions: EntityStore::new(),
        }
    }
}
