use bevy::{
	ecs::{
		query::With,
		system::{Query, Single},
	},
	ui::widget::Text,
};

use crate::plugins::evaluator::{
	components::{
		debugger_context::DebuggerContextComponent,
		highlighted_node_description::HighlightedNodeDescriptionComponent, node::NodeComponent,
		node_highlighted::NodeHighlightedComponent,
	},
	node::Node,
};

pub fn update_highlighted_node_description(
	context: Single<&DebuggerContextComponent>,
	highlighted_nodes_query: Query<&NodeComponent, With<NodeHighlightedComponent>>,
	description: Single<&mut Text, With<HighlightedNodeDescriptionComponent>>,
) {
	let mut description = description.into_inner();
	if let Some(highlighted_node) = highlighted_nodes_query.into_iter().next() {
		let context = context.into_inner();
		let context = context.with_context();
		match &highlighted_node.node().node {
			Node::Hypothesis(index, _) => {
				description.0 = format!("{}", context.hypotheses()[*index]);
			}
			Node::Desire(index) => {
				description.0 = format!("{}", context.desires()[*index]);
			}
		}

		return;
	}

	description.0 = "".into();
}
