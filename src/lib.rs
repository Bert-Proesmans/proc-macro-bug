#[macro_use]
extern crate action_macros;
extern crate action_traits;

use action_traits::{Actionable, Triggerable};

#[derive(Debug, ActionState, TriggerState)]
pub struct EndTurn();
