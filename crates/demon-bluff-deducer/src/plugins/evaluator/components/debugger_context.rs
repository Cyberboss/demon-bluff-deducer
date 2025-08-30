use std::{
	collections::HashMap,
	sync::{Arc, RwLock, RwLockReadGuard},
};

use bevy::{
	color::{Color, Mix},
	ecs::component::Component,
	gizmos::gizmos::Gizmos,
	math::{Isometry2d, Rot2, Vec2},
	platform::collections::HashSet,
	time::Time,
};
use demon_bluff_logic_engine::{DebuggerContext, FITNESS_UNIMPLEMENTED};
use force_graph::{DefaultNodeIdx, EdgeData, ForceGraph, NodeData};

use crate::plugins::evaluator::{
	colours::{
		COLOUR_DESIRE_NEGATIVE, COLOUR_DESIRE_POSITIVE, COLOUR_HIGHLIGHT,
		COLOUR_HYPOTHESIS_NEGATIVE, COLOUR_HYPOTHESIS_POSITIVE, COLOUR_NEUTRAL,
		COLOUR_UNIMPLEMENTED, COLOUR_VISITING,
	},
	edge::Edge,
	node::Node,
	node_data::NodeAndLocked,
	node_radius::NodeRadius,
};

#[derive(Component)]
pub struct DebuggerContextComponent {
	debug_context: Arc<RwLock<DebuggerContext>>,
	graph: ForceGraph<NodeAndLocked, Edge>,
	hypothesis_map: HashMap<usize, DefaultNodeIdx>,
	desire_map: HashMap<usize, DefaultNodeIdx>,
	current_hypothesis_path: Vec<usize>,
	current_hypothesis_path_set: HashSet<usize>,
}

impl DebuggerContextComponent {
	pub fn new(debug_context: Arc<RwLock<DebuggerContext>>) -> Self {
		Self {
			debug_context,
			graph: ForceGraph::new(Default::default()),
			hypothesis_map: HashMap::new(),
			desire_map: HashMap::new(),
			current_hypothesis_path: Vec::new(),
			current_hypothesis_path_set: HashSet::new(),
		}
	}

	pub fn register_hypothesis(&mut self, index: usize, is_root: bool) -> &NodeData<NodeAndLocked> {
		let guard = self
			.debug_context
			.read()
			.expect("Debugger context was poisoned!");
		let dependent_hypotheses = guard.hypotheses()[index].hypothesis_dependencies().len();
		let graph_index = self.graph.add_node(NodeData {
			x: 1.0 * index as f32,
			y: 0.0,
			mass: if is_root {
				50.0
			} else {
				1.0 + dependent_hypotheses as f32
			},
			is_anchor: is_root,
			user_data: NodeAndLocked {
				node: Node::Hypothesis(index, is_root),
				locked_coordinate: None,
			},
		});

		self.hypothesis_map.insert(index, graph_index);

		&self.graph.get_graph()[graph_index].data
	}

	pub fn register_desire(&mut self, index: usize) -> &NodeData<NodeAndLocked> {
		let guard = self
			.debug_context
			.read()
			.expect("Debugger context was poisoned!");
		let dependent_hypotheses = guard
			.hypotheses()
			.iter()
			.flat_map(|hypothesis_node| {
				hypothesis_node
					.desire_consumer_dependencies()
					.iter()
					.chain(hypothesis_node.desire_producer_dependencies())
					.filter(|desire_dependency| **desire_dependency == index)
			})
			.count();
		let graph_index = self.graph.add_node(NodeData {
			x: 0.0,
			y: 1.0 * index as f32,
			mass: 1.0 + dependent_hypotheses as f32,
			is_anchor: false,
			user_data: NodeAndLocked {
				node: Node::Desire(index),
				locked_coordinate: None,
			},
		});
		self.desire_map.insert(index, graph_index);
		&self.graph.get_graph()[graph_index].data
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
					.get(dep)
					.expect("This hypothesis index was not mapped in the graph!");

				self.graph.add_edge(
					*dependency_hypothesis_node_index,
					*hypothesis_node_index,
					EdgeData {
						user_data: Edge::Hypothesis(*dep, index),
					},
				);
			}

			for dep in hypo_node.desire_consumer_dependencies() {
				let desire_node_index = self
					.desire_map
					.get(dep)
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
					.get(dep)
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

	pub fn update_graph(&mut self, time: &Time) {
		self.graph.update(time.delta_secs());

		self.graph.visit_nodes_mut(|node| {
			if let Some(locked_coordinate) = node.data.user_data.locked_coordinate {
				node.data.x = locked_coordinate.x;
				node.data.y = locked_coordinate.y;
			}
		});
	}

	pub fn with_context<'a>(&'a self) -> RwLockReadGuard<'a, DebuggerContext> {
		self.debug_context
			.read()
			.expect("Debugger context was poisoned!")
	}

	pub fn update_node_entity<'a>(
		&'a self,
		context: &DebuggerContext,
		node: &Node,
	) -> (Color, Vec2, Option<f32>) {
		let (graph_index, visiting, is_root) = match node {
			Node::Hypothesis(index, is_root_inner) => (
				self.hypothesis_map
					.get(index)
					.expect("Requested update for node that wasn't registered!"),
				self.current_hypothesis_path_set.contains(index),
				*is_root_inner,
			),
			Node::Desire(index) => (
				self.desire_map
					.get(index)
					.expect("Requested update for node that wasn't registered!"),
				false,
				false,
			),
		};

		let node = &self.graph.get_graph()[*graph_index];

		let (negative_colour, positive_colour, fitness) = match node.data.user_data.node {
			Node::Hypothesis(index, _) => (
				COLOUR_HYPOTHESIS_NEGATIVE,
				COLOUR_HYPOTHESIS_POSITIVE,
				context.hypotheses()[index]
					.current_fitness()
					.map(|fitness_and_action| fitness_and_action.fitness()),
			),
			Node::Desire(index) => (
				COLOUR_DESIRE_NEGATIVE,
				COLOUR_DESIRE_POSITIVE,
				Some(context.desires()[index].fitness_value()),
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

		let vec = Vec2::new(node.x(), node.y());

		let highlight_radius = if visiting {
			Some(node.data.radius(is_root))
		} else {
			None
		};

		(color, vec, highlight_radius)
	}

	pub fn apply_node_delta(&mut self, node: &Node, delta: Vec2, lock: bool) {
		match node {
			Node::Hypothesis(_, is_root) => {
				if *is_root {
					return;
				}
			}
			Node::Desire(_) => {}
		}

		self.graph.visit_nodes_mut(|visited_node| {
			let visited_node_data = &mut visited_node.data;
			if visited_node_data.user_data.node == *node {
				visited_node_data.x += delta.x;
				visited_node_data.y -= delta.y; // why -? no fucking clue
				visited_node_data.user_data.locked_coordinate = if lock {
					Some(Vec2::new(visited_node_data.x, visited_node_data.y))
				} else {
					None
				}
			}
		});
	}

	pub fn draw_highlight(&self, node: &Node, gizmos: &mut Gizmos) {
		let is_root;
		let graph_idx = match node {
			Node::Hypothesis(index, is_root_inner) => {
				is_root = *is_root_inner;
				self.hypothesis_map.get(index)
			}
			Node::Desire(index) => {
				is_root = false;
				self.desire_map.get(index)
			}
		}
		.expect("Attempting to highlight unregistered node!");

		let data = &self.graph.get_graph()[*graph_idx].data;
		let radius = data.radius(is_root);

		gizmos.circle_2d(
			Isometry2d::from_translation(Vec2::new(data.x, data.y)),
			radius,
			COLOUR_HIGHLIGHT,
		);
	}

	pub fn draw_edges(&self, mut gizmos: Gizmos) {
		let guard = self
			.debug_context
			.read()
			.expect("Debugger context was poisoned!");

		let clockwise_rotation = Rot2::degrees(-30.0);
		let counterclockwise_rotation = Rot2::degrees(30.0);
		self.graph.visit_edges(|dependency, dependent, edge| {
			let dependency_vec = Vec2::new(dependency.x(), dependency.y());
			let dependent_vec = Vec2::new(dependent.x(), dependent.y());

			let dependent_is_root =
				if let Node::Hypothesis(_, is_root) = dependent.data.user_data.node {
					is_root
				} else {
					false
				};

			let dependent_vec = dependent_vec
				.move_towards(dependency_vec, dependent.data.radius(dependent_is_root));
			let dependency_vec =
				dependency_vec.move_towards(dependent_vec, dependency.data.radius(false));

			let colour = match edge.user_data {
				Edge::Hypothesis(dependency_hypothesis_index, dependent_hypothesis_index) => {
					let dependency = &guard.hypotheses()[dependency_hypothesis_index];

					let visiting = self
						.current_hypothesis_path_set
						.contains(&dependency_hypothesis_index)
						&& self
							.current_hypothesis_path_set
							.contains(&dependent_hypothesis_index);

					if visiting {
						COLOUR_VISITING
					} else {
						match dependency.current_fitness() {
							Some(fitness_and_action) => COLOUR_HYPOTHESIS_NEGATIVE.mix(
								&COLOUR_HYPOTHESIS_POSITIVE,
								fitness_and_action.fitness() as f32,
							),
							None => COLOUR_NEUTRAL,
						}
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

	pub fn enter_hypothesis(&mut self, index: usize) {
		self.current_hypothesis_path.push(index);
		self.current_hypothesis_path_set.insert(index);
	}

	pub fn exit_hypothesis(&mut self) {
		if let Some(removed_index) = self.current_hypothesis_path.pop() {
			self.current_hypothesis_path_set.remove(&removed_index);
		}
	}
}
