use demon_bluff_gameplay_engine::{
    Expression,
    game_state::GameState,
    testimony::{ConfessorClaim, Testimony},
    villager::{Villager, VillagerIndex},
};
use log::Log;

use super::{DesireType, HypothesisBuilderType, HypothesisType};
use crate::engine::{
    Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
    HypothesisFunctions, HypothesisRegistrar, HypothesisRepository,
    HypothesisResult,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TestimonyCondemnsHypothesisBuilder {
    testifier: VillagerIndex,
    defendant: VillagerIndex,
    testimony: Expression<Testimony>,
}

impl TestimonyCondemnsHypothesisBuilder {
    pub fn new(
        testifier: VillagerIndex,
        testimony: Expression<Testimony>,
        defendant: VillagerIndex,
    ) -> Self {
        Self {
            testifier,
            defendant,
            testimony,
        }
    }
}

/// If a testimony is true and condemns a given defendent
#[derive(Debug)]
pub struct TestimonyCondemnsHypothesis {
    testifier: VillagerIndex,
    defendant: VillagerIndex,
}

impl HypothesisBuilder for TestimonyCondemnsHypothesisBuilder {
    fn build(
        self,
        _: &GameState,
        _: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
        TestimonyCondemnsHypothesis {
            testifier: self.testifier,
            defendant: self.defendant,
        }
        .into()
    }
}

impl Hypothesis for TestimonyCondemnsHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}'s testimony condemns {}",
            self.testifier, self.defendant
        )
    }

    fn wip(&self) -> bool {
        true
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        game_state: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisEvaluation
    where
        TLog: Log,
    {
        let testimony = match game_state.villager(&self.testifier) {
            Villager::Active(active_villager) => active_villager.instance().testimony(),
            Villager::Hidden(_) => {
                return repository
                    .finalize(HypothesisResult::Conclusive(FitnessAndAction::impossible()));
            }
            Villager::Confirmed(confirmed_villager) => confirmed_villager.instance().testimony(),
        };

        let expression = match testimony {
            Some(testimony) => testimony,
            None => {
                return repository
                    .finalize(HypothesisResult::Conclusive(FitnessAndAction::impossible()));
            }
        };

        // TODO: Make this use a testimony_condemns_expression and then just match on the testimony

        match expression {
            Expression::Unary(Testimony::Confess(confession)) => {
                if self.defendant == self.testifier {
                    repository.finalize(HypothesisResult::Conclusive(match confession {
                        ConfessorClaim::Good => FitnessAndAction::impossible(),
                        ConfessorClaim::Dizzy => FitnessAndAction::certainty(None),
                    }))
                } else {
                    repository
                        .finalize(HypothesisResult::Conclusive(FitnessAndAction::impossible()))
                }
            }
            _ => {
                repository.finalize(HypothesisResult::Conclusive(
                    FitnessAndAction::unimplemented(),
                ))
            }
        }
    }
}
