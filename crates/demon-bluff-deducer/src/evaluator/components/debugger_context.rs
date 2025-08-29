use std::{
	collections::{HashMap, HashSet},
	sync::{Arc, Mutex, MutexGuard},
};

use bevy::{
	animation::graph,
	color::{
		Color, Mix,
		color_difference::{self, EuclideanDistance},
	},
	ecs::component::Component,
	gizmos::gizmos::Gizmos,
	math::{Isometry2d, Vec2},
	tasks::Task,
	time::Time,
};
use demon_bluff_logic_engine::{
	DebuggerContext, FITNESS_UNIMPLEMENTED, PlayerAction, PredictionError,
};
use force_graph::{DefaultNodeIdx, EdgeData, ForceGraph, NodeData};

use crate::evaluator::{
	colours::{
		COLOUR_DESIRE_NEGATIVE, COLOUR_DESIRE_POSITIVE, COLOUR_HYPOTHESIS_NEGATIVE,
		COLOUR_HYPOTHESIS_POSITIVE, COLOUR_NEUTRAL, COLOUR_UNIMPLEMENTED,
	},
	edge::Edge,
	edge_link::EdgeLink,
	node::Node,
};

#[derive(Component)]
pub struct DebuggerContextComponent {
	debug_context: Arc<Mutex<DebuggerContext>>,
	graph: ForceGraph<Node, Edge>,
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

	pub fn register_hypothesis(&mut self, index: usize, is_root: bool) {
		let graph_index = self.graph.add_node(NodeData {
			x: 1.0 * index as f32,
			y: 0.0,
			mass: if is_root { 50.0 } else { 1.0 },
			is_anchor: is_root,
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
			.expect("Debugger context was poisoned!");
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

				self.graph.add_edge(
					*lhs,
					*rhs,
					EdgeData {
						user_data: Edge::Hypothesis(*dep),
					},
				);
			}

			for dep in hypo_node.desire_consumer_dependencies() {
				let rhs = self
					.desire_map
					.get(&dep)
					.expect("This desire index was not mapped in the graph!");

				self.graph.add_edge(
					*lhs,
					*rhs,
					EdgeData {
						user_data: Edge::DesireConsumer(*dep),
					},
				);
			}

			for dep in hypo_node.desire_producer_dependencies() {
				let rhs = self
					.desire_map
					.get(&dep)
					.expect("This desire index was not mapped in the graph!");

				self.graph.add_edge(
					*lhs,
					*rhs,
					EdgeData {
						user_data: Edge::DesireProducer(*dep, None),
					},
				);
			}
		}
	}

	pub fn update_desire_producer(
		&mut self,
		producer_hypothesis_index: usize,
		desire_index: usize,
		desired: bool,
	) {
		self.graph.add_edge(
			*self
				.hypothesis_map
				.get(&producer_hypothesis_index)
				.expect("Invalid hypothesis index provided!"),
			*self
				.desire_map
				.get(&desire_index)
				.expect("Invalid hypothesis index provided!"),
			EdgeData {
				user_data: Edge::DesireProducer(desire_index, Some(desired)),
			},
		);
	}

	pub fn update_and_draw_graph(&mut self, mut gizmos: Gizmos, time: &Time) {
		let guard = self
			.debug_context
			.lock()
			.expect("Debugger context was poisoned!");
		self.graph.update(time.delta_secs());
		self.graph.visit_nodes(|node| {
			let (negative_colour, positive_colour, fitness) = match node.data.user_data {
				Node::Hypothesis(index) => (
					COLOUR_HYPOTHESIS_NEGATIVE,
					COLOUR_HYPOTHESIS_POSITIVE,
					guard.hypotheses()[index]
						.current_fitness()
						.map(|fitness_and_action| fitness_and_action.fitness()),
				),
				Node::Desire(index) => (
					COLOUR_DESIRE_NEGATIVE,
					COLOUR_DESIRE_POSITIVE,
					Some(guard.desires()[index].fitness_value()),
				),
			};

			let color = match fitness {
				Some(fitness) => {
					if fitness == FITNESS_UNIMPLEMENTED {
						COLOUR_UNIMPLEMENTED
					} else {
						negative_colour.mix(&positive_colour, fitness as f32)
					}
				}
				None => COLOUR_NEUTRAL,
			};

			gizmos.circle_2d(
				Isometry2d::from_translation(Vec2::new(node.x(), node.y())),
				5.0,
				color,
			);
		});

		self.graph.visit_edges(|lhs, rhs, edge| {
			gizmos.line_2d(
				Vec2::new(lhs.x(), lhs.y()),
				Vec2::new(rhs.x(), rhs.y()),
				match edge.user_data {
					Edge::Hypothesis(dependency_hypothesis_index) => {
						let dependency = &guard.hypotheses()[dependency_hypothesis_index];

						match dependency.current_fitness() {
							Some(fitness_and_action) => COLOUR_HYPOTHESIS_NEGATIVE.mix(
								&COLOUR_HYPOTHESIS_POSITIVE,
								fitness_and_action.fitness() as f32,
							),
							None => COLOUR_NEUTRAL,
						}
					}
					Edge::DesireProducer(_, desired) => match desired {
						Some(desired) => {
							if desired {
								COLOUR_DESIRE_POSITIVE
							} else {
								COLOUR_DESIRE_NEGATIVE
							}
						}
						None => COLOUR_NEUTRAL,
					},
					Edge::DesireConsumer(desire_index) => {
						let desire = &guard.desires()[desire_index];
						COLOUR_DESIRE_NEGATIVE
							.mix(&COLOUR_DESIRE_POSITIVE, desire.fitness_value() as f32)
					}
				},
			);
		});
	}
}
