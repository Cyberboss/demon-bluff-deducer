use bevy::prelude::*;

use super::{
	state::EvaluatorState,
	systems::{
		camera_controls::camera_controls, check_for_break::check_for_break,
		check_for_resume::check_for_resume, draw_graph_edges::draw_graph_edges,
		get_prediction_result::get_prediction_result, init_evaluation::init_evaluation,
		update_graph::update_graph, update_graph_from_breakpoint::update_graph_from_breakpoint,
		update_node_entities::update_node_entities,
	},
};
use crate::root_state::RootState;

pub struct EvaluatorPlugin;

impl Plugin for EvaluatorPlugin {
	fn build(&self, app: &mut App) {
		app
			// At start, the menu is not enabled. This will be changed in `menu_setup` when
			// entering the `GameState::Menu` state.
			// Current screen in the menu is handled by an independent state from `GameState`
			.add_sub_state::<EvaluatorState>()
			.add_systems(OnEnter(RootState::Evaluation), init_evaluation)
			.add_systems(
				OnEnter(EvaluatorState::Break),
				update_graph_from_breakpoint.after(check_for_break),
			)
			.add_systems(
				Update,
				(
					camera_controls,
					update_graph.after(update_graph_from_breakpoint),
					(draw_graph_edges, update_node_entities).after(update_graph),
					(check_for_break, get_prediction_result)
						.run_if(in_state(EvaluatorState::Running)),
					check_for_resume
						.after(update_graph_from_breakpoint)
						.run_if(in_state(EvaluatorState::Break)),
				),
			);
	}
}
