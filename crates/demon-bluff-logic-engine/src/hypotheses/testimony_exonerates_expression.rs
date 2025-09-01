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
		HypothesisRepository, and_result, or_result,
	},
	hypotheses::{
		HypothesisType, hypothesis_expression::HypothesisExpression,
		testimony_exonerates::TestimonyExoneratesHypothesisBuilder,
		testimony_expression::TestimonyExpressionHypothesisBuilder,
	},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TestimonyExoneratesExpressionHypothesisBuilder {
	testifier: VillagerIndex,
	defendant: VillagerIndex,
	testimony_expression: Expression<Testimony>,
	is_root_testimony: bool,
}

/// If a given [`Testimony`] [`Expression`] is true AND exonerates a given defendant
#[derive(Debug)]
pub struct TestimonyExoneratesExpressionHypothesis {
	testifier: VillagerIndex,
	defendant: VillagerIndex,
	exonerates_expression: HypothesisExpression,
	truthfulness_hypothesis: HypothesisReference,
	expression_friendly: String,
	is_root_testimony: bool,
}

impl TestimonyExoneratesExpressionHypothesisBuilder {
	pub fn new(
		testifier: VillagerIndex,
		defendant: VillagerIndex,
		testimony_expression: Expression<Testimony>,
	) -> Self {
		Self {
			testifier,
			defendant,
			testimony_expression,
			is_root_testimony: true,
		}
	}

	fn sub_new(
		testifier: VillagerIndex,
		defendant: VillagerIndex,
		testimony_expression: Expression<Testimony>,
	) -> Self {
		Self {
			testifier,
			defendant,
			testimony_expression,
			is_root_testimony: false,
		}
	}
}

impl HypothesisBuilder for TestimonyExoneratesExpressionHypothesisBuilder {
	fn build(
		self,
		_: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		let expression_friendly = format!("{}", self.testimony_expression);
		let exonerates_expression = match &self.testimony_expression {
			Expression::Unary(testimony) => HypothesisExpression::Unary(registrar.register(
				TestimonyExoneratesHypothesisBuilder::new(
					self.testifier.clone(),
					self.defendant.clone(),
					testimony.clone(),
				),
			)),
			Expression::Not(expression) => HypothesisExpression::Not(registrar.register(
				TestimonyExoneratesExpressionHypothesisBuilder::sub_new(
					self.testifier.clone(),
					self.defendant.clone(),
					*expression.clone(),
				),
			)),
			Expression::And(lhs, rhs) => HypothesisExpression::And((
				registrar.register(TestimonyExoneratesExpressionHypothesisBuilder::sub_new(
					self.testifier.clone(),
					self.defendant.clone(),
					*lhs.clone(),
				)),
				registrar.register(TestimonyExoneratesExpressionHypothesisBuilder::sub_new(
					self.testifier.clone(),
					self.defendant.clone(),
					*rhs.clone(),
				)),
			)),
			Expression::Or(lhs, rhs) => HypothesisExpression::Or((
				registrar.register(TestimonyExoneratesExpressionHypothesisBuilder::sub_new(
					self.testifier.clone(),
					self.defendant.clone(),
					*lhs.clone(),
				)),
				registrar.register(TestimonyExoneratesExpressionHypothesisBuilder::sub_new(
					self.testifier.clone(),
					self.defendant.clone(),
					*rhs.clone(),
				)),
			)),
		};

		let truthfulness_hypothesis =
			registrar.register(TestimonyExpressionHypothesisBuilder::new_with_root_status(
				self.testifier.clone(),
				self.testimony_expression,
				self.is_root_testimony,
			));

		TestimonyExoneratesExpressionHypothesis {
			testifier: self.testifier.clone(),
			defendant: self.defendant.clone(),
			expression_friendly,
			exonerates_expression,
			truthfulness_hypothesis,
			is_root_testimony: self.is_root_testimony,
		}
		.into()
	}
}

impl Hypothesis for TestimonyExoneratesExpressionHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		if self.is_root_testimony {
			write!(f, "Root t")?
		} else {
			write!(f, "Sub-t")?
		}

		write!(
			f,
			"estimony expression from {} exonerates {}: {}",
			self.testifier, self.defendant, self.expression_friendly
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
		let testimony_condemns_result = match &self.exonerates_expression {
			HypothesisExpression::Unary(hypothesis_reference) => {
				evaluator.sub_evaluate(hypothesis_reference)
			}
			HypothesisExpression::Not(hypothesis_reference) => evaluator
				.sub_evaluate(hypothesis_reference)
				.map(|fitness_and_action| fitness_and_action.invert()),
			HypothesisExpression::And((lhs, rhs)) => {
				// remember, we're not testing for truthfulness, just if the expression condemns the target in any way
				or_result(evaluator.sub_evaluate(lhs), evaluator.sub_evaluate(rhs))
			}
			HypothesisExpression::Or((lhs, rhs)) => {
				// I'm not a statistics person, but after a long chat with LLMs they're insistent that this is the correct operation
				// It makes sense, being the opposite of the above
				and_result(evaluator.sub_evaluate(lhs), evaluator.sub_evaluate(rhs))
			}
		};

		let testimony_true_result = evaluator.sub_evaluate(&self.truthfulness_hypothesis);

		let result = and_result(testimony_true_result, testimony_condemns_result);

		evaluator.finalize(result)
	}
}
