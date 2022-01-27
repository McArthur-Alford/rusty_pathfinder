// use std::{time::*};

// #[derive(Debug)]
// pub struct WorldTime {
//     pub start: SystemTime,
//     pub passed: Duration,
// }

// impl Default for WorldTime {
//     fn default() -> Self {
//         Self {
//             start: SystemTime::now(),
//             passed: Duration::new(0, 0),
//         }
//     }
// }

// impl WorldTime {
//     pub fn real_time_tick(&mut self) {
//         match self.start.elapsed() {
//             Err(_) => (),
//             Ok(D) => self.passed += D,
//         };
//         self.start = SystemTime::now();
//     }

//     pub fn turn_time_tick(&mut self) {
//         self.passed += Duration::from_secs(6);
//     }
// }

// pub enum EffectDuration {
//     Permenant,
//     Seconds { start: Duration, duration: Duration },
// }
