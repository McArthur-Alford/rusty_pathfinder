use hecs::Entity;

type EntityId = usize;

pub struct EntityStore {
    entities_count: EntityId,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl EntityStore {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    pub fn add_entity(&mut self) -> EntityId {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    pub fn remove_entity(
        &mut self,
        entity: EntityId,
        
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.set_none(entity);
        }
    }

    pub fn add_component<ComponentType: 'static>(
        &mut self,
        entity: EntityId,
        component: ComponentType,
    ) {
        if entity >= self.entities_count {
            return;
        }

        // Check all component vecs for a valid store and insert:
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec.insert(entity, Some(component));
                return;
            }
        }

        // The component vec doesn't exist, so we create a new empty one:
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        // Set the component at our entity index in the vec:
        new_component_vec[entity] = Some(component);

        // Put the new vec in the EntityStore:
        self.component_vecs.push(Box::new(new_component_vec));
    }

    pub fn remove_component<ComponentType: 'static>(&mut self, entity: EntityId) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity] = None;
                return;
            }
        }
    }

    pub fn get_component<ComponentType: 'static>(&mut self, entity: EntityId) -> Option<&ComponentType> {
        if entity >= self.entities_count {
            return None;
        }

        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                return component_vec[entity].as_ref();
            }
        }
        None
    }
}

pub trait ComponentVec {
    fn push_none(&mut self);
    fn set_none(&mut self, entity: EntityId);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> ComponentVec for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None);
    }

    fn set_none(&mut self, entity: EntityId) {
        self.insert(entity, None);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}