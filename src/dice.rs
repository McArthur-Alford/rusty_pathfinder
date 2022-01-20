// use rand::{prelude::thread_rng, Rng};
// use specs::{hibitset::BitSetLike, prelude::Resource};
// use std::ops::Range;

// pub enum Dice {
//     Range(Range<i32>),
//     D(i32),
//     Percentile,
// }

// #[derive(Debug)]
// pub enum SuccessStep {
//     CritFail,
//     Fail,
//     Success,
//     CritSuccess,
// }

// #[derive(Debug)]
// pub struct RollResult {
//     success: SuccessStep,
//     difference: i32,
//     result: i32,
//     natural_crit: bool
// }

// impl RollResult {
//     fn new(success: SuccessStep, difference: i32, result: i32, natural_crit: bool) -> RollResult {
//         RollResult {
//             success,
//             difference,
//             result,
//             natural_crit
//         }
//     }
// }

// pub struct DiceManager;

// impl Resource for DiceManager {}

// impl DiceManager {
//     pub fn roll(&self, dice: Dice) -> i32 {
//         let mut rng = thread_rng();
//         match dice {
//             Dice::Range(range) => rng.gen_range(range),
//             Dice::D(sides) => rng.gen_range(1..=sides),
//             Dice::Percentile => rng.gen_range(1..=100),
//         }
//     }

//     pub fn check(&self, dice: Dice, dc: i32) -> RollResult {
//         let result = self.roll(dice);
//         let difference = result - dc;
//         let mut success = match difference {
//             10..=i32::MAX => 3,
//             0..=9 => 2,
//             -9..=-1 => 1,
//             i32::MIN..=-10 => 0
//         };
//         success += match result {
//             20 => 1,
//             1 => -1,
//             _ => 0
//         };
//         let success = match success {
//             -1..=0 => SuccessStep::CritFail,
//             1 => SuccessStep::Fail,
//             2 => SuccessStep::Success,
//             3..=4 => SuccessStep::CritSuccess,
//             _ => unreachable!()
//         };
//         RollResult::new(success, difference, result, result == 20 || result == 1)
//     }
// }
