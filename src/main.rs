/** Sample Combat Rounds:
 *
 * Everyone rolls initiative and gets added to schedule.
 *
 * Battle Round 1 Starts, World Time Progresses by 0 Seconds.
 * The scheduler passes delta_t to all decorator systems.
 *
 * Turn 1 - Fighter
 * Fighter uses charge action.
 *  - Creates a blank action.
 *  - Passes the action through a move_builder and attack_builder or something.
 *  - The action describes the final changes to the gamestate.
 *      = The player is adjacent to the enemy.
 *      = The result of the attack is applied to the enemy.
 * The threat range rules check this charge and find the fighter moves through the threat range of a goblin to get there.
 * The rules ask the goblin ai for intent.
 *  - A AI player would have functions
 *  - A human player would have the game briefly pause and wait for input
 * The goblin AI returns a request for an attack_action coming from the enum of action names/descriptors.
 * The rule that queries the goblin rejects the original charge action.
 *  - Creates a move to the point where the player is threatened by the goblin.
 *  - Creates an attack from the goblin against the player.
 *  - Creates a move from the threatened tile to the charge location.
 *  - Creates a new attack against the target.
 * The new action order looks like this:
 * Move to Threatened Square -> Goblin Attacks Player -> Move to Destination -> Attack
 * 
 * Lets assume no other rules are violated. and skip to the goblin attacking the player:
 * The next action is Attack. This request enum variant stores attackerId, SingleTarget(Id), attackId (reference to attack data on the attacker).
 * A blank action is created.
 *  - It is populated with an attack using the data in the enum request.
 *  - Presumably I have skipped over a roll action, which this would use data from.
 *  - The action describes the changed gamestate (assuming a hit, the target has less health, etc).
 * The defender has shield block. The shield block rule detects this feat and that an ActionType: Attack against it being processed.
 * It queries the defender AI for intent (the player). Assume the player doesn't accept.
 *
 * The fighter has 1 action so the scheduler keeps calling the fighter on repeat. 
 * The fighter uses a pass turn action, this action specifically sets the actions remaining to 0.
 *
 * The scheduler sees this on the next game tick moves to Round 1, Turn 2
 * ...
 * **/
use entity_store::*;

mod entity_store;

// struct Action {
//     insertions: Vec<Box<dyn ComponentVec>>
// }

fn main() {
    let mut world = EntityStore::new();
    world.set_component::<i32>(0, 1);
    world.set_component::<i32>(0, 2);
    world.set_none::<i32>(0);
    println!("{:?}", world.get_component::<i32>(0));

    let mut action = Action::new();
    action.insertions.set_component::<i32>(0, 3);
    world.commit(&mut action);
    println!("{:?}", world.get_component::<i32>(0));
}
