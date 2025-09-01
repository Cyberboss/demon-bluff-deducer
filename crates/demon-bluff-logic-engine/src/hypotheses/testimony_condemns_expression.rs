use std::collections::HashMap;

use demon_bluff_gameplay_engine::{
	Expression, game_state::GameState, testimony::Testimony, villager::VillagerIndex,
};
use log::Log;

use super::{DesireType, HypothesisBuilderType};
use crate::{
	Breakpoint,
	engine::{
		Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
		HypothesisEvaluator, HypothesisFunctions, HypothesisReference, HypothesisRegistrar,
		HypothesisRepository,
	},
	expression_assertion::probability_expression_asserts_x_given_true,
	hypotheses::{HypothesisType, testimony_condemns::TestimonyCondemnsHypothesisBuilder},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TestimonyCondemnsExpressionHypothesisBuilder {
	testifier: VillagerIndex,
	defendant: VillagerIndex,
	testimony_expression: Expression<Testimony>,
}

/// If a given [`Testimony`] [`Expression`] condemns a give defendant ASSUMING it is true
#[derive(Debug)]
pub struct TestimonyCondemnsExpressionHypothesis {
	testifier: VillagerIndex,
	defendant: VillagerIndex,
	testimony_expression: Expression<Testimony>,
	component_condemns_hypotheses: HashMap<Testimony, HypothesisReference>,
}

impl TestimonyCondemnsExpressionHypothesisBuilder {
	pub fn new(
		testifier: VillagerIndex,
		defendant: VillagerIndex,
		testimony_expression: Expression<Testimony>,
	) -> Self {
		Self {
			testifier,
			defendant,
			testimony_expression,
		}
	}
}

impl HypothesisBuilder for TestimonyCondemnsExpressionHypothesisBuilder {
	fn build(
		self,
		_: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		let mut component_condemns_hypotheses = HashMap::new();
		self.build_component_condemns_hypotheses(
			&mut component_condemns_hypotheses,
			registrar,
			&self.testimony_expression,
		);

		TestimonyCondemnsExpressionHypothesis {
			testifier: self.testifier,
			defendant: self.defendant,
			testimony_expression: self.testimony_expression,
			component_condemns_hypotheses,
		}
		.into()
	}
}

impl Hypothesis for TestimonyCondemnsExpressionHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f,
			"Testimony expression from {} condemns {}: {}",
			self.testifier, self.defendant, self.testimony_expression
		)
	}

	fn evaluate<TLog, FDebugBreak>(
		&mut self,
		_: &TLog,
		_: Depth,
		_: &GameState,
		repository: HypothesisRepository<TLog, FDebugBreak>,
	) -> HypothesisEvaluation
	where
		TLog: Log,
		FDebugBreak: FnMut(Breakpoint) + Clone,
	{
		let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

		let mut evaluations = HashMap::new();
		for (testimony, hypothesis_reference) in &self.component_condemns_hypotheses {
			evaluations.insert(
				testimony.clone(),
				evaluator.sub_evaluate(hypothesis_reference),
			);
		}

		let result =
			probability_expression_asserts_x_given_true(&self.testimony_expression, &evaluations);

		evaluator.finalize(result)
	}
}

impl TestimonyCondemnsExpressionHypothesisBuilder {
	/// current navigation is the navigation to expression
	fn build_component_condemns_hypotheses(
		&self,
		component_condemns_hypotheses: &mut HashMap<Testimony, HypothesisReference>,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
		expression: &Expression<Testimony>,
	) {
		match expression {
			Expression::Leaf(testimony) => {
				component_condemns_hypotheses.insert(
					testimony.clone(),
					registrar.register(TestimonyCondemnsHypothesisBuilder::new(
						self.testifier.clone(),
						self.defendant.clone(),
						testimony.clone(),
					)),
				);
			}
			Expression::And(lhs, rhs) | Expression::Or(lhs, rhs) => {
				self.build_component_condemns_hypotheses(
					component_condemns_hypotheses,
					registrar,
					&lhs,
				);
				self.build_component_condemns_hypotheses(
					component_condemns_hypotheses,
					registrar,
					&rhs,
				);
			}
		}
	}
}
