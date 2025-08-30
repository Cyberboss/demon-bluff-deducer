use std::ops::Index;

use bevy::{
	ecs::{event::EventWriter, system::Single},
	log::warn,
};
use demon_bluff_logic_engine::Breakpoint;
use force_graph::NodeData;

use crate::plugins::evaluator::{
	components::{breakpoint::BreakpointComponent, debugger_context::DebuggerContextComponent},
	events::node_spawn::NodeSpawnEvent,
	node::Node,
};

pub fn update_graph_from_breakpoint(
	mut node_spawn_writer: EventWriter<NodeSpawnEvent>,
	breakpoint: Single<&BreakpointComponent>,
	context: Single<&mut DebuggerContextComponent>,
) {
	let mut context = context.into_inner();
	match breakpoint.breakpoint() {
		Breakpoint::Initialize(_) => {}
		Breakpoint::RegisterHypothesis(index, root) => {
			let node = context.register_hypothesis(*index, *root);
			node_spawn_writer.write(NodeSpawnEvent::new(clone_node_data(node), *root));
		}
		Breakpoint::RegisterDesire(index) => {
			let node = context.register_desire(*index);
			node_spawn_writer.write(NodeSpawnEvent::new(clone_node_data(node), false));
		}
		Breakpoint::IterationStart(iteration) => {
			if *iteration == 1 {
				context.finalize_edges();
			}
		}
		Breakpoint::DesireUpdate(producer_hypothesis_index, desire_index, desired) => {
			context.update_desire_producer(*producer_hypothesis_index, *desire_index, *desired)
		}
		Breakpoint::EnterHypothesis(index) => context.enter_hypothesis(*index),
		Breakpoint::ExitHypothesis(_) => context.exit_hypothesis(),
		_ => {
			warn!(
				"Unhandled graph update from breakpoint: {}",
				breakpoint.breakpoint()
			);
		}
	}
}

fn clone_node_data(this: &NodeData<Node>) -> NodeData<Node> {
	NodeData {
		x: this.x.clone(),
		y: this.y.clone(),
		mass: this.mass.clone(),
		is_anchor: this.is_anchor.clone(),
		user_data: this.user_data.clone(),
	}
}
