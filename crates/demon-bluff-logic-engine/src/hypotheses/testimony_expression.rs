use demon_bluff_gameplay_engine::{
	Expression, game_state::GameState, testimony::Testimony, villager::VillagerIndex,
};
use log::Log;

use super::{DesireType, HypothesisBuilderType, hypothesis_expression::HypothesisExpression};
use crate::{
	Breakpoint,
	engine::{
		Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
		HypothesisEvaluator, HypothesisFunctions, HypothesisRegistrar, HypothesisRepository,
		and_result, or_result,
	},
	hypotheses::{HypothesisType, testimony::TestimonyHypothesisBuilder},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TestimonyExpressionHypothesisBuilder {
	index: VillagerIndex,
	testimony_expression: Expression<Testimony>,
	is_root_testimony: bool,
}

#[derive(Debug)]
pub struct TestimonyExpressionHypothesis {
	index: VillagerIndex,
	hypothesis_expression: HypothesisExpression,
	expression_friendly: String,
	is_root_testimony: bool,
}

impl TestimonyExpressionHypothesisBuilder {
	pub fn new(index: VillagerIndex, testimony_expression: Expression<Testimony>) -> Self {
		Self {
			index,
			testimony_expression,
			is_root_testimony: true,
		}
	}

	fn sub_new(index: VillagerIndex, testimony_expression: Expression<Testimony>) -> Self {
		Self {
			index,
			testimony_expression,
			is_root_testimony: false,
		}
	}

	pub fn new_with_root_status(
		index: VillagerIndex,
		testimony_expression: Expression<Testimony>,
		is_root_testimony: bool,
	) -> Self {
		if is_root_testimony {
			Self::new(index, testimony_expression)
		} else {
			Self::sub_new(index, testimony_expression)
		}
	}
}

impl HypothesisBuilder for TestimonyExpressionHypothesisBuilder {
	fn build(
		self,
		_: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		let expression_friendly = format!("{}", self.testimony_expression);
		let hypothesis_expression = match self.testimony_expression {
			Expression::Leaf(testimony) => HypothesisExpression::Unary(registrar.register(
				TestimonyHypothesisBuilder::new(self.index.clone(), testimony.clone()),
			)),
			Expression::And(lhs, rhs) => HypothesisExpression::And((
				registrar.register(TestimonyExpressionHypothesisBuilder::sub_new(
					self.index.clone(),
					*lhs.clone(),
				)),
				registrar.register(TestimonyExpressionHypothesisBuilder::sub_new(
					self.index.clone(),
					*rhs.clone(),
				)),
			)),
			Expression::Or(lhs, rhs) => HypothesisExpression::Or((
				registrar.register(TestimonyExpressionHypothesisBuilder::sub_new(
					self.index.clone(),
					*lhs.clone(),
				)),
				registrar.register(TestimonyExpressionHypothesisBuilder::sub_new(
					self.index.clone(),
					*rhs.clone(),
				)),
			)),
		};

		TestimonyExpressionHypothesis {
			index: self.index,
			expression_friendly,
			hypothesis_expression,
			is_root_testimony: self.is_root_testimony,
		}
		.into()
	}
}

impl Hypothesis for TestimonyExpressionHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		if self.is_root_testimony {
			write!(f, "Root t")?
		} else {
			write!(f, "Sub-t")?
		}

		write!(
			f,
			"estimony expression from {}: {}",
			self.index, self.expression_friendly
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
		let result = match &self.hypothesis_expression {
			HypothesisExpression::Unary(hypothesis_reference) => {
				evaluator.sub_evaluate(hypothesis_reference)
			}
			HypothesisExpression::Not(hypothesis_reference) => evaluator
				.sub_evaluate(hypothesis_reference)
				.map(|fitness_and_action| fitness_and_action.invert()),
			HypothesisExpression::And((lhs, rhs)) => {
				and_result(evaluator.sub_evaluate(lhs), evaluator.sub_evaluate(rhs))
			}
			HypothesisExpression::Or((lhs, rhs)) => {
				or_result(evaluator.sub_evaluate(lhs), evaluator.sub_evaluate(rhs))
			}
		};

		evaluator.finalize(result)
	}
}
