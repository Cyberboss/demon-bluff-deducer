use bevy::prelude::*;

use super::{
	state::EvaluatorState,
	systems::{
		check_for_break::check_for_break, check_for_resume::check_for_resume,
		draw_graph::draw_graph, init_evaluation::init_evaluation,
		update_graph_from_breakpoint::update_graph_from_breakpoint,
	},
};
use crate::state::RootState;

pub struct EvaluatorPlugin;

impl Plugin for EvaluatorPlugin {
	fn build(&self, app: &mut App) {
		app
			// At start, the menu is not enabled. This will be changed in `menu_setup` when
			// entering the `GameState::Menu` state.
			// Current screen in the menu is handled by an independent state from `GameState`
			.add_sub_state::<EvaluatorState>()
			.add_systems(OnEnter(RootState::Evaluation), init_evaluation)
			.add_systems(OnEnter(EvaluatorState::Break), update_graph_from_breakpoint)
			.add_systems(
				Update,
				(
					draw_graph,
					check_for_break.run_if(in_state(EvaluatorState::Running)),
					check_for_resume.run_if(in_state(EvaluatorState::Break)),
				),
			);
	}
}
