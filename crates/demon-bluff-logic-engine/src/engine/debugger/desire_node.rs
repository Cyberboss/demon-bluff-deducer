use std::fmt::Display;

use crate::{engine::DesireProducerReference, hypotheses::DesireType};

#[derive(Debug)]
pub struct DesireNode {
	reference: DesireProducerReference,
	desire_type: DesireType,
	pending: usize,
	desired: usize,
	undesired: usize,
}

impl DesireNode {
	fn new(reference: DesireProducerReference, desire_type: DesireType, producers: usize) -> Self {
		Self {
			reference,
			desire_type,
			pending: producers,
			desired: 0,
			undesired: 0,
		}
	}

	pub fn pending(&self) -> usize {
		self.pending
	}

	pub fn total(&self) -> usize {
		self.desired + self.pending + self.undesired
	}

	pub fn fitness_value(&self) -> f64 {
		self.desired as f64 / self.total() as f64
	}
}

impl Display for DesireNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}: {} ({}/{})",
			self.reference,
			self.desire_type,
			self.desired,
			self.total()
		)?;

		if self.pending > 0 {
			write!(f, " ({} Pending)", self.pending)
		} else {
			Ok(())
		}
	}
}

pub fn create_desire_node(
	reference: DesireProducerReference,
	desire_type: DesireType,
	producers: usize,
) -> DesireNode {
	DesireNode::new(reference, desire_type, producers)
}

pub fn update_desire_node(node: &mut DesireNode, pending: usize, desired: usize, undesired: usize) {
	node.pending = pending;
	node.desired = desired;
	node.undesired = undesired;
}
