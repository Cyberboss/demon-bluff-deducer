use std::fmt::Display;

use demon_bluff_gameplay_engine::{
	Expression,
	testimony::{self, Testimony},
	villager::{ConfirmedVillager, VillagerIndex},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IndexTestimony {
	pub index: VillagerIndex,
	pub testimony: Testimony,
}

impl IndexTestimony {
	pub fn new(index: VillagerIndex, testimony: Testimony) -> Self {
		Self { index, testimony }
	}
}

impl Display for IndexTestimony {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} says: {}", self.index, self.testimony)
	}
}

pub fn build_expression_for_villager_set<'a>(
	confirmeds: impl IntoIterator<Item = &'a ConfirmedVillager>,
) -> Option<Expression<IndexTestimony>> {
	let mut expression = None;
	for (index, confirmed) in confirmeds.into_iter().enumerate() {
		let testimony_expression = match confirmed.instance().testimony() {
			Some(testimony) => Some(
				if !confirmed.instance().archetype().cannot_lie()
					&& (confirmed.corrupted() || confirmed.true_identity().lies())
				{
					Expression::Not(Box::new(map_testimony(testimony, &VillagerIndex(index))))
				} else {
					map_testimony(testimony, &VillagerIndex(index))
				},
			),
			None => None,
		};

		expression = match expression {
			Some(existing_expression) => Some(match testimony_expression {
				Some(testimony_expression) => Expression::And(
					Box::new(existing_expression),
					Box::new(testimony_expression),
				),
				None => existing_expression,
			}),
			None => testimony_expression,
		}
	}

	expression
}

fn map_testimony(
	original: &Expression<Testimony>,
	index: &VillagerIndex,
) -> Expression<IndexTestimony> {
	match original {
		Expression::Leaf(testimony) => {
			Expression::Leaf(IndexTestimony::new(index.clone(), testimony.clone()))
		}
		Expression::Not(expression) => Expression::Not(Box::new(map_testimony(expression, index))),
		Expression::And(lhs, rhs) => Expression::And(
			Box::new(map_testimony(lhs, index)),
			Box::new(map_testimony(rhs, index)),
		),
		Expression::Or(lhs, rhs) => Expression::Or(
			Box::new(map_testimony(lhs, index)),
			Box::new(map_testimony(rhs, index)),
		),
	}
}
