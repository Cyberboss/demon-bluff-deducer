use std::{
	collections::{HashMap, HashSet},
	sync::{Arc, Mutex, MutexGuard, RwLock},
};

use bevy::{
	animation::graph,
	color::{
		Color, Mix,
		color_difference::{self, EuclideanDistance},
	},
	ecs::component::Component,
	gizmos::gizmos::Gizmos,
	math::{Isometry2d, Quat, Rot2, Vec2},
	tasks::Task,
	time::Time,
};
use demon_bluff_logic_engine::{
	DebuggerContext, FITNESS_UNIMPLEMENTED, PlayerAction, PredictionError,
};
use force_graph::{DefaultNodeIdx, EdgeData, ForceGraph, NodeData};

use crate::plugins::evaluator::{
	colours::{
		COLOUR_DESIRE_NEGATIVE, COLOUR_DESIRE_POSITIVE, COLOUR_HYPOTHESIS_NEGATIVE,
		COLOUR_HYPOTHESIS_POSITIVE, COLOUR_NEUTRAL, COLOUR_UNIMPLEMENTED,
	},
	edge::Edge,
	node::Node,
};

#[derive(Component)]
pub struct DebuggerContextComponent {
	debug_context: Arc<RwLock<DebuggerContext>>,
	graph: ForceGraph<Node, Edge>,
	hypothesis_map: HashMap<usize, DefaultNodeIdx>,
	desire_map: HashMap<usize, DefaultNodeIdx>,
	root_index: usize,
}

impl DebuggerContextComponent {
	pub fn new(debug_context: Arc<RwLock<DebuggerContext>>) -> Self {
		Self {
			debug_context,
			graph: ForceGraph::new(Default::default()),
			hypothesis_map: HashMap::new(),
			desire_map: HashMap::new(),
			root_index: 0,
		}
	}

	pub fn register_hypothesis(&mut self, index: usize, is_root: bool) {
		let guard = self
			.debug_context
			.read()
			.expect("Debugger context was poisoned!");
		let dependent_hypotheses = guard.hypotheses()[index].hypothesis_dependencies().len();
		let graph_index = self.graph.add_node(NodeData {
			x: 1.0 * index as f32,
			y: 0.0,
			mass: if is_root {
				self.root_index = index;
				50.0
			} else {
				1.0 + dependent_hypotheses as f32
			},
			is_anchor: is_root,
			user_data: Node::Hypothesis(index),
		});

		self.hypothesis_map.insert(index, graph_index);
	}

	pub fn register_desire(&mut self, index: usize) {
		let guard = self
			.debug_context
			.read()
			.expect("Debugger context was poisoned!");
		let dependent_hypotheses = guard
			.hypotheses()
			.iter()
			.map(|hypothesis_node| {
				hypothesis_node
					.desire_consumer_dependencies()
					.iter()
					.chain(hypothesis_node.desire_producer_dependencies())
					.filter(|desire_dependency| **desire_dependency == index)
			})
			.flatten()
			.count();
		let graph_index = self.graph.add_node(NodeData {
			x: 0.0,
			y: 1.0 * index as f32,
			mass: 1.0 + dependent_hypotheses as f32,
			is_anchor: false,
			user_data: Node::Desire(index),
		});
		self.desire_map.insert(index, graph_index);
	}

	pub fn finalize_edges(&mut self) {
		let guard = self
			.debug_context
			.read()
			.expect("Debugger context was poisoned!");
		for (index, hypo_node) in guard.hypotheses().iter().enumerate() {
			let hypothesis_node_index = self
				.hypothesis_map
				.get(&index)
				.expect("This hypothesis index was not mapped in the graph!");

			for dep in hypo_node.hypothesis_dependencies() {
				let dependency_hypothesis_node_index = self
					.hypothesis_map
					.get(&dep)
					.expect("This hypothesis index was not mapped in the graph!");

				self.graph.add_edge(
					*dependency_hypothesis_node_index,
					*hypothesis_node_index,
					EdgeData {
						user_data: Edge::Hypothesis(*dep),
					},
				);
			}

			for dep in hypo_node.desire_consumer_dependencies() {
				let desire_node_index = self
					.desire_map
					.get(&dep)
					.expect("This desire index was not mapped in the graph!");

				self.graph.add_edge(
					*desire_node_index,
					*hypothesis_node_index,
					EdgeData {
						user_data: Edge::DesireConsumer(*dep),
					},
				);
			}

			for dep in hypo_node.desire_producer_dependencies() {
				let desire_node_index = self
					.desire_map
					.get(&dep)
					.expect("This desire index was not mapped in the graph!");

				self.graph.add_edge(
					*hypothesis_node_index,
					*desire_node_index,
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
			.read()
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
				5.0 * if node.data.user_data == Node::Hypothesis(self.root_index) {
					1.0
				} else {
					node.data.mass
				},
				color,
			);
		});

		let clockwise_rotation = Rot2::degrees(-30.0);
		let counterclockwise_rotation = Rot2::degrees(30.0);
		self.graph.visit_edges(|lhs, rhs, edge| {
			let dependency_vec = Vec2::new(lhs.x(), lhs.y());
			let dependent_vec = Vec2::new(rhs.x(), rhs.y());
			let colour = match edge.user_data {
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
			};
			gizmos.line_2d(dependency_vec, dependent_vec, colour);

			let midpoint_vec = (dependency_vec + dependent_vec) / 2.0;
			let ray = dependency_vec - midpoint_vec;
			let absolute_ray = ray.normalize();
			let sized_ray = absolute_ray * 5.0;

			let arrow_ray_clockwise = clockwise_rotation * sized_ray;
			let arrow_ray_counterclockwise = counterclockwise_rotation * sized_ray;

			let arrow_dash_clockwise = midpoint_vec + arrow_ray_clockwise;
			let arrow_dash_counterclockwise = midpoint_vec + arrow_ray_counterclockwise;

			gizmos.line_2d(arrow_dash_clockwise, midpoint_vec, colour);
			gizmos.line_2d(arrow_dash_counterclockwise, midpoint_vec, colour);
		});
	}
}
