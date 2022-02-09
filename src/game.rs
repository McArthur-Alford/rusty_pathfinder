use crate::entity_store::{Action, ActionTag, EntityId, EntityStore};
use crate::rule::{Rule, RuleResult};
use crate::Pos;

pub struct Game {
    world: EntityStore,
    rules: Vec<&'static Rule>,
    // scheduler: Scheduler
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: EntityStore::new(),
            rules: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        // loop {
        self.main_loop();
        // }
    }

    fn main_loop(&mut self) {
        self.world.set_component::<Pos>(0, Pos::new(10.0, 10.0));
        // Creates a position component type as this is the first time one is used.
        // Sets the position component of entity 0 to (10.0, 10.0).

        let mut action = Action::new();
        // Create an empty action.

        move_position(&self.world, &mut action, 0, Pos::new(5.0,0.0));
        move_position(&self.world, &mut action, 0, Pos::new(5.0,0.0));
        // Modifies the action to describe a move by 10 to the right for entity 0.
        // Note that breakig it into two seperate "blocks" has no effect.
        // Rules will only see the result.
        // This means things like chained actions/long movements should be seperate.
        // e.g. a 30" move in pathfinder would be 6 5" move actions scheduled in chain.

        for rule in &mut self.rules {
            match rule(&self.world, &action) {
                RuleResult::Success => continue,
                RuleResult::Reject => continue
            }
        }




        println!("{:?}", self.world.get_component::<Pos>(0));
        // Output: Some(Vec2F32 { x: 10.0, y: 10.0 })
        // Notice the action hasn't been applied.

        println!(
            "{:?}",
            EntityStore::get_future_component::<Pos>(0, &mut self.world, &mut action)
        );
        // Output: Some(Vec2F32 { x: 0.0, y: 10.0 })
        // While the action hasn't been applied we can act as if it has by querying the future.

        self.world.commit(&mut action);
        // Commit the action to the world.

        println!("{:?}", self.world.get_component::<Pos>(0));
        // Output: Some(Vec2F32 { x: 0.0, y: 10.0 })
    }
}

fn move_position(world: &EntityStore, action: &mut Action, entity: EntityId, direction: Pos) {
    let position = EntityStore::get_future_component::<Pos>(entity, world, action);
    if let Some(position) = position {
        let new_position = *position + direction;
        action.insertions.set_component::<Pos>(entity, new_position);
    }
    action.give_tag(ActionTag::Movement);
}
