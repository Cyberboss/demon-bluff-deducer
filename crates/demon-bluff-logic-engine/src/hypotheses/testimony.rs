use demon_bluff_gameplay_engine::{
    game_state::GameState,
    testimony::Testimony,
    villager::{Outcast, VillagerArchetype, VillagerIndex},
};
use log::Log;

use super::{DesireType, HypothesisBuilderType};
use crate::{
    engine::{
        Depth, FITNESS_UNKNOWN, FitnessAndAction, Hypothesis, HypothesisBuilder,
        HypothesisEvaluation, HypothesisEvaluator, HypothesisFunctions, HypothesisReference,
        HypothesisRegistrar, HypothesisRepository, HypothesisResult,
    },
    hypotheses::{
        HypothesisType, appears_evil::AppearsEvilHypothesisBuilder,
        is_corrupt::IsCorruptHypothesisBuilder, is_evil::IsEvilHypothesisBuilder,
        is_truly_archetype::IsTrulyArchetypeHypothesisBuilder,
        is_truthful::IsTruthfulHypothesisBuilder, negate::NegateHypothesisBuilder,
    },
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TestimonyHypothesisBuilder {
    index: VillagerIndex,
    testimony: Testimony,
}

/// Checks the validity of a [`Testimony`]` from a [`VillagerIndex`] based on factors other than the villager lying
#[derive(Debug)]
pub struct TestimonyHypothesis {
    index: VillagerIndex,
    testimony: Testimony,
    villager_sub_hypothesis: Option<HypothesisReference>,
}

impl TestimonyHypothesisBuilder {
    pub fn new(index: VillagerIndex, testimony: Testimony) -> Self {
        Self { index, testimony }
    }
}

impl HypothesisBuilder for TestimonyHypothesisBuilder {
    fn build(
        self,
        _: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
        let villager_sub_hypothesis_builder: Option<HypothesisBuilderType> = match &self.testimony {
            Testimony::Good(villager_index) => Some(
                NegateHypothesisBuilder::new(IsEvilHypothesisBuilder::new(villager_index.clone()))
                    .into(),
            ),
            Testimony::Slayed(slay_result) => {
                if slay_result.slayed() {
                    None
                } else {
                    Some(
                        NegateHypothesisBuilder::new(IsEvilHypothesisBuilder::new(
                            slay_result.index().clone(),
                        ))
                        .into(),
                    )
                }
            }
            Testimony::Evil(villager_index) => {
                Some(AppearsEvilHypothesisBuilder::new(villager_index.clone()).into())
            }
            Testimony::Corrupt(villager_index) => {
                Some(IsCorruptHypothesisBuilder::new(villager_index.clone()).into())
            }
            Testimony::NotCorrupt(villager_index) => Some(
                NegateHypothesisBuilder::new(IsCorruptHypothesisBuilder::new(
                    villager_index.clone(),
                ))
                .into(),
            ),
            Testimony::Lying(villager_index) => Some(
                NegateHypothesisBuilder::new(IsTruthfulHypothesisBuilder::new(
                    villager_index.clone(),
                ))
                .into(),
            ),
            Testimony::Role(role_claim) => Some(
                IsTrulyArchetypeHypothesisBuilder::new(
                    role_claim.role().clone(),
                    role_claim.index().clone(),
                )
                .into(),
            ),
            Testimony::SelfDestruct(villager_index) => Some(
                IsTrulyArchetypeHypothesisBuilder::new(
                    VillagerArchetype::Outcast(Outcast::Bombardier),
                    villager_index.clone(),
                )
                .into(),
            ),
            Testimony::Confess(_) => None,
            _ => todo!(),
        };

        let villager_sub_hypothesis =
            villager_sub_hypothesis_builder.map(|builder| registrar.register_builder_type(builder));
        TestimonyHypothesis {
            index: self.index,
            testimony: self.testimony,
            villager_sub_hypothesis,
        }
        .into()
    }
}

impl Hypothesis for TestimonyHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Testimony from {}: {}", self.index, self.testimony)
    }

    fn wip(&self) -> bool {
        true
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        _: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisEvaluation
    where
        TLog: Log,
    {
        match &self.testimony {
            Testimony::Confess(_) => repository.finalize(HypothesisResult::Conclusive(
                FitnessAndAction::certainty(None),
            )),
            Testimony::Good(_)
            | Testimony::Evil(_)
            | Testimony::Corrupt(_)
            | Testimony::Lying(_)
            | Testimony::NotCorrupt(_)
            | Testimony::Role(_)
            | Testimony::SelfDestruct(_) => {
                let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);
                let result = evaluator.sub_evaluate(
                    self.villager_sub_hypothesis.as_ref().unwrap_or_else(|| {
                        panic!(
                            "We didn't get a sub hypothesis to evaluate for {}'s testimony: {}",
                            self.index, self.testimony
                        );
                    }),
                );
                evaluator.finalize(result)
            }
            Testimony::Slayed(slay_result) => {
                if slay_result.slayed() {
                    return repository.finalize(HypothesisResult::Conclusive(
                        FitnessAndAction::certainty(None),
                    ));
                }

                let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);
                let result = evaluator.sub_evaluate(
                    self.villager_sub_hypothesis.as_ref().unwrap_or_else(|| {
                        panic!(
                            "We didn't get a sub hypothesis to evaluate for {}'s slay testimony: {}",
                            self.index, self.testimony
                        );
                    }),
                );
                evaluator.finalize(result)
            }
            _ => repository.finalize(HypothesisResult::unimplemented()),
        }
    }
}
