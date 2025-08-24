use demon_bluff_gameplay_engine::{
    Expression,
    game_state::GameState,
    testimony::{self, Testimony},
    villager::{Villager, VillagerIndex},
};
use log::Log;

use crate::{
    hypotheses::{
        HypothesisType,
        testimony::{TestimonyHypothesis, TestimonyHypothesisBuilder},
        testimony_expression,
    },
    hypothesis::{
        Depth, FITNESS_UNKNOWN, FitnessAndAction, Hypothesis, HypothesisBuilder,
        HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisResult,
        HypothesisReturn, and_result, or_result,
    },
};

#[derive(Debug)]
enum HypothesisExpression {
    Unary(HypothesisReference),
    Not(HypothesisReference),
    And((HypothesisReference, HypothesisReference)),
    Or((HypothesisReference, HypothesisReference)),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TestimonyExpressionHypothesisBuilder {
    index: VillagerIndex,
    testimony_expression: Expression<Testimony>,
}

#[derive(Debug)]
pub struct TestimonyExpressionHypothesis {
    index: VillagerIndex,
    hypothesis_expression: HypothesisExpression,
    expression_friendly: String,
}

impl TestimonyExpressionHypothesisBuilder {
    pub fn new(index: VillagerIndex, testimony_expression: Expression<Testimony>) -> Self {
        Self {
            index,
            testimony_expression,
        }
    }
}

impl HypothesisBuilder for TestimonyExpressionHypothesisBuilder {
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let expression_friendly = format!("{}", self.testimony_expression);
        let hypothesis_expression = match self.testimony_expression {
            Expression::Unary(testimony) => HypothesisExpression::Unary(registrar.register(
                TestimonyHypothesisBuilder::new(self.index.clone(), testimony.clone()),
            )),
            Expression::Not(expression) => HypothesisExpression::Not(registrar.register(
                TestimonyExpressionHypothesisBuilder::new(self.index.clone(), *expression.clone()),
            )),
            Expression::And(lhs, rhs) => HypothesisExpression::And((
                registrar.register(TestimonyExpressionHypothesisBuilder::new(
                    self.index.clone(),
                    *lhs.clone(),
                )),
                registrar.register(TestimonyExpressionHypothesisBuilder::new(
                    self.index.clone(),
                    *rhs.clone(),
                )),
            )),
            Expression::Or(lhs, rhs) => HypothesisExpression::Or((
                registrar.register(TestimonyExpressionHypothesisBuilder::new(
                    self.index.clone(),
                    *lhs.clone(),
                )),
                registrar.register(TestimonyExpressionHypothesisBuilder::new(
                    self.index.clone(),
                    *rhs.clone(),
                )),
            )),
        };

        TestimonyExpressionHypothesis {
            index: self.index,
            expression_friendly,
            hypothesis_expression,
        }
        .into()
    }
}

impl Hypothesis for TestimonyExpressionHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Evaluate testimony expression from {}: {}",
            self.index, self.expression_friendly
        )
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        _: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
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

        evaluator.create_return(result)
    }
}
