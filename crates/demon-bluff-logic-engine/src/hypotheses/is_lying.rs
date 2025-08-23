use demon_bluff_gameplay_engine::{
    Expression,
    game_state::GameState,
    testimony::Testimony,
    villager::{Villager, VillagerIndex},
};
use log::Log;

use crate::{
    hypotheses::testimony_expression::TestimonyExpressionHypothesis,
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
};

#[derive(Eq, PartialEq, Debug)]
pub struct IsLyingHypothesis {
    index: VillagerIndex,
    testimony_expression_hypothesis: Option<HypothesisReference>,
}

impl IsLyingHypothesis {
    pub fn create<TLog>(
        game_state: &GameState,
        mut registrar: &mut HypothesisRegistrar<TLog>,
        index: VillagerIndex,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        let testimony_expression_hypothesis = match game_state.villager(&index) {
            Villager::Active(active_villager) => match active_villager.instance().testimony() {
                Some(expression) => {
                    let negated_expression = Expression::Not(Box::new(expression.clone()));
                    Some(TestimonyExpressionHypothesis::create(
                        game_state,
                        registrar,
                        &negated_expression,
                    ))
                }
                None => None,
            },
            Villager::Hidden(_) | Villager::Confirmed(_) => None,
        };
        registrar.register(Self {
            index,
            testimony_expression_hypothesis,
        })
    }
}

impl Hypothesis for IsLyingHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} is lying", self.index)
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
