use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum RootState {
	#[default]
	Menu,
	Setup,
	Evaluation,
	AutoPlay,
}
