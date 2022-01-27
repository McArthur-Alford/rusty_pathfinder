// use crate::document::Document;

// struct Initiative(u32);

// trait Action {
//     fn Enact(&self, source: &mut Document, target: &mut Document) -> &mut ActionResult;
// }

// enum ActionResult {
//     Success(u32),                      // Returnedaction succeeds or no refund on fail
//     Alternative(Box<dyn Action>), // Returned if the action calls another action
//     Fail,                         // Returned if action failure refunds the action
// }

// fn apply_action(mut action: &mut Box<dyn Action>, source: &mut Document, target: &mut Document) -> u32 {
//     loop {
//         match action.Enact(source, target) {
//             ActionResult::Success(cost) => return * cost,
//             ActionResult::Alternative(A) => action = A,
//             ActionResult::Fail => return 0,
//         };
//     }
// }




// struct AttackAction {

// }

// impl Action for AttackAction {
//     fn Enact(&self, source: &mut Document, target: &mut Document) -> &mut ActionResult {
//         source.world.
//     }
// }