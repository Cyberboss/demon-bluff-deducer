use bevy::prelude::*;

use super::{state::EvaluatorState, systems::init_evaluation::init_evaluation};
use crate::state::RootState;

pub struct EvaluatorPlugin;

impl Plugin for EvaluatorPlugin {
	fn build(&self, app: &mut App) {
		app
			// At start, the menu is not enabled. This will be changed in `menu_setup` when
			// entering the `GameState::Menu` state.
			// Current screen in the menu is handled by an independent state from `GameState`
			.add_sub_state::<EvaluatorState>()
			.add_systems(OnEnter(RootState::Evaluation), init_evaluation);
	}
}
