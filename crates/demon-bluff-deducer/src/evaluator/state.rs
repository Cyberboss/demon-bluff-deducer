use bevy::prelude::*;

use crate::root_state::RootState;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, SubStates)]
#[source(RootState = RootState::Menu)]
pub enum EvaluatorState {
	#[default]
	Running,
	Break,
}
