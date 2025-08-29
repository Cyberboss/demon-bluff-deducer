use bevy::prelude::*;

use crate::state::RootState;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, SubStates)]
#[source(RootState = RootState::Evaluation)]
pub enum EvaluatorState {
	#[default]
	Running,
	Break,
	Complete,
}
