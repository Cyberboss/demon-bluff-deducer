use demon_bluff_gameplay_engine::{
    Expression,
    game_state::GameState,
    testimony::{self, Testimony},
    villager::{Villager, VillagerIndex},
};
use log::Log;

use crate::{
    hypotheses::testimony::TestimonyHypothesis,
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
};

#[derive(Eq, PartialEq, Debug)]
enum HypothesisExpression {
    Unary(HypothesisReference),
    Not(HypothesisReference),
    And((HypothesisReference, HypothesisReference)),
    Or((HypothesisReference, HypothesisReference)),
}

#[derive(Eq, PartialEq, Debug)]
pub struct TestimonyExpressionHypothesis {
    hypothesis_expression: HypothesisExpression,
    expression_friendly: String,
}

impl TestimonyExpressionHypothesis {
    pub fn create<TLog>(
        game_state: &GameState,
        mut registrar: &mut HypothesisRegistrar<TLog>,
        expression: &Expression<Testimony>,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        let expression_friendly = format!("{}", expression);
        let hypothesis_expression = match expression {
            Expression::Unary(testimony) => HypothesisExpression::Unary(
                TestimonyHypothesis::create(game_state, &mut registrar, testimony.clone()),
            ),
            Expression::Not(expression) => HypothesisExpression::Not(
                TestimonyExpressionHypothesis::create(game_state, registrar, expression),
            ),
            Expression::And(lhs, rhs) => HypothesisExpression::And((
                TestimonyExpressionHypothesis::create(game_state, registrar, lhs),
                TestimonyExpressionHypothesis::create(game_state, registrar, rhs),
            )),
            Expression::Or(lhs, rhs) => HypothesisExpression::Or((
                TestimonyExpressionHypothesis::create(game_state, registrar, lhs),
                TestimonyExpressionHypothesis::create(game_state, registrar, rhs),
            )),
        };
        registrar.register(Self {
            hypothesis_expression,
            expression_friendly,
        })
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
        repository.create_return(HypothesisResult::Conclusive(
            FitnessAndAction::unimplemented(),
        ))
    }
}
