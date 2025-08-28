use std::{
	collections::{HashMap, HashSet},
	sync::{Arc, Mutex},
};

use bevy::{
	animation::graph,
	color::Color,
	ecs::component::Component,
	gizmos::gizmos::Gizmos,
	math::{Isometry2d, Vec2},
	tasks::Task,
	time::Time,
};
use demon_bluff_logic_engine::{DebuggerContext, PlayerAction, PredictionError};
use force_graph::{DefaultNodeIdx, ForceGraph, NodeData};

use super::node::Node;

#[derive(Component)]
pub struct DebuggerContextComponent {
	debug_context: Arc<Mutex<DebuggerContext>>,
	graph: ForceGraph<Node>,
	hypothesis_map: HashMap<usize, DefaultNodeIdx>,
	desire_map: HashMap<usize, DefaultNodeIdx>,
}

impl DebuggerContextComponent {
	pub fn new(debug_context: Arc<Mutex<DebuggerContext>>) -> Self {
		Self {
			debug_context,
			graph: ForceGraph::new(Default::default()),
			hypothesis_map: HashMap::new(),
			desire_map: HashMap::new(),
		}
	}

	pub fn register_hypothesis(&mut self, index: usize) {
		let graph_index = self.graph.add_node(NodeData {
			x: 1.0 * index as f32,
			y: 0.0,
			mass: 1.0,
			is_anchor: false,
			user_data: Node::Hypothesis(index),
		});
		self.hypothesis_map.insert(index, graph_index);
	}

	pub fn register_desire(&mut self, index: usize) {
		let graph_index = self.graph.add_node(NodeData {
			x: 0.0,
			y: 1.0 * index as f32,
			mass: 1.0,
			is_anchor: false,
			user_data: Node::Desire(index),
		});
		self.desire_map.insert(index, graph_index);
	}

	pub fn register_edges(&mut self) {
		let guard = self
			.debug_context
			.lock()
			.expect("Unable to lock debug context");
		for (index, hypo_node) in guard.hypotheses().iter().enumerate() {
			let lhs = self
				.hypothesis_map
				.get(&index)
				.expect("This hypothesis index was not mapped in the graph!");

			for dep in hypo_node.hypothesis_dependencies() {
				let rhs = self
					.hypothesis_map
					.get(&dep)
					.expect("This hypothesis index was not mapped in the graph!");

				self.graph.add_edge(*lhs, *rhs, Default::default());
			}
		}
	}

	pub fn update_and_draw_graph(&mut self, mut gizmos: Gizmos, time: &Time) {
		self.graph.update(time.delta_secs());
		self.graph.visit_nodes(|node| {
			gizmos.circle_2d(
				Isometry2d::from_translation(Vec2::new(node.x(), node.y())),
				5.0,
				Color::srgb(1.0, 0.0, 0.0),
			);
		});

		self.graph.visit_edges(|lhs, rhs, _| {
			gizmos.line_2d(
				Vec2::new(lhs.x(), lhs.y()),
				Vec2::new(rhs.x(), rhs.y()),
				Color::srgb(0.0, 0.0, 1.0),
			);
		});
	}
}
