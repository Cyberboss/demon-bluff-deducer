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
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisReference,
        HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn,
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
    testimony_expression: Expression<Testimony>,
}

#[derive(Debug)]
pub struct TestimonyExpressionHypothesis {
    hypothesis_expression: HypothesisExpression,
    expression_friendly: String,
}

impl TestimonyExpressionHypothesisBuilder {
    pub fn new(testimony_expression: Expression<Testimony>) -> Self {
        Self {
            testimony_expression,
        }
    }
}

impl HypothesisBuilder for TestimonyExpressionHypothesisBuilder {
    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let expression_friendly = format!("{}", self.testimony_expression);
        let hypothesis_expression = match self.testimony_expression {
            Expression::Unary(testimony) => HypothesisExpression::Unary(
                registrar.register(TestimonyHypothesisBuilder::new(testimony.clone())),
            ),
            Expression::Not(expression) => HypothesisExpression::Not(registrar.register(
                TestimonyExpressionHypothesisBuilder::new(*expression.clone()),
            )),
            Expression::And(lhs, rhs) => HypothesisExpression::And((
                registrar.register(TestimonyExpressionHypothesisBuilder::new(*lhs.clone())),
                registrar.register(TestimonyExpressionHypothesisBuilder::new(*rhs.clone())),
            )),
            Expression::Or(lhs, rhs) => HypothesisExpression::Or((
                registrar.register(TestimonyExpressionHypothesisBuilder::new(*lhs.clone())),
                registrar.register(TestimonyExpressionHypothesisBuilder::new(*rhs.clone())),
            )),
        };

        TestimonyExpressionHypothesis {
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
            "Evaluate testimony expression: {}",
            self.expression_friendly
        )
    }

    fn wip(&self) -> bool {
        true
    }

    fn evaluate<TLog>(
        &mut self,
        log: &TLog,
        depth: Depth,
        game_state: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        repository.create_return(HypothesisResult::unimplemented())
    }
}
