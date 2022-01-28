/** Sample Combat Rounds:
 *
 * Everyone rolls initiative and gets added to schedule.
 *
 * Battle Round 1 Starts, World Time Progresses by 0 Seconds.
 * The scheduler passes delta_t to all decorator systems.
 *
 * Turn 1 - Fighter
 * Fighter uses charge action. This composes a move and attack together.
 * The threat range rules check this charge and find the fighter moves through the threat range of a goblin.
 * The rules ask the goblin AI and receive an attackaction against the player.
 * The rules reject the original action and adjust the queue from:
 * Move to Destination -> Attack
 * To:
 * Move to Threatened Square -> Goblin Attacks Player -> Move to Destination -> Attack
 * Assume everything up to move to destination just happens for the example.
 *
 * The next action is Attack. This enum stores attackerId, SingleTarget(Id).
 * It runs through an attack builder enum. This calculates the dice rolls, etc,
 *  and builds the changed gamestate (assuming a hit, the target has less health).
 * The defender has shield block. The shield block rule detects this feat, and an ActionType: Attack against it being processed.
 *  It queries the defender AI for intent. Assuming true, it sets up a new queue, else it accepts the status quo.
 *
 * The fighter has 1 action left and uses the pass action.
 * - The pass action reduces the available actions to 0. No rules complain.
 *
 * The game tick finishes the action queue (outside of example scope, assume empty).
 *
 * The scheduler moves to Round 1, Turn 2
 * ...
 * **/
use entity_store::*;

mod entity_store;

struct Action {
    insertions: Vec<Box<dyn ComponentVec>>
}

fn main() {
    let mut world = EntityStore::new();
    let entity0 = world.add_entity();
    world.add_component(entity0, 1);
    println!("{:?}", world.get_component::<i32>(1));

}
