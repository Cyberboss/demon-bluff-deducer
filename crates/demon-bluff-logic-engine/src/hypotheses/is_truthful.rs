use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{Villager, VillagerIndex},
};
use log::Log;

use super::{DesireType, HypothesisBuilderType, desires::GetTestimonyDesire};
use crate::{
    engine::{
        Depth, DesireProducerReference, FITNESS_UNKNOWN, FitnessAndAction, Hypothesis,
        HypothesisBuilder, HypothesisEvaluation, HypothesisEvaluator, HypothesisFunctions,
        HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisResult,
    },
    hypotheses::{HypothesisType, testimony_expression::TestimonyExpressionHypothesisBuilder},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct IsTruthfulHypothesisBuilder {
    index: VillagerIndex,
}

#[derive(Debug)]
enum SubReferenceType {
    CheckTestimony(HypothesisReference),
    GetTestimony(DesireProducerReference),
}

#[derive(Debug)]
pub struct IsTruthfulHypothesis {
    index: VillagerIndex,
    sub_reference: Option<SubReferenceType>,
}

impl IsTruthfulHypothesisBuilder {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl HypothesisBuilder for IsTruthfulHypothesisBuilder {
    fn build(
        self,
        game_state: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
        let sub_reference = match game_state.villager(&self.index) {
            Villager::Active(active_villager) => {
                Some(match active_villager.instance().testimony() {
                    Some(testimony) => SubReferenceType::CheckTestimony(registrar.register(
                        TestimonyExpressionHypothesisBuilder::new(
                            self.index.clone(),
                            testimony.clone(),
                        ),
                    )),
                    None => SubReferenceType::GetTestimony(registrar.register_desire_producer(
                        DesireType::GetTestimony(GetTestimonyDesire::new(self.index.clone())),
                    )),
                })
            }
            Villager::Hidden(_) => Some(SubReferenceType::GetTestimony(
                registrar.register_desire_producer(DesireType::GetTestimony(
                    GetTestimonyDesire::new(self.index.clone()),
                )),
            )),
            Villager::Confirmed(confirmed_villager) => {
                match confirmed_villager.instance().testimony() {
                    Some(_) => None,
                    None => Some(SubReferenceType::GetTestimony(
                        registrar.register_desire_producer(DesireType::GetTestimony(
                            GetTestimonyDesire::new(self.index.clone()),
                        )),
                    )),
                }
            }
        };

        IsTruthfulHypothesis {
            index: self.index,
            sub_reference,
        }
        .into()
    }
}

impl Hypothesis for IsTruthfulHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} is truthful", self.index)
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        game_state: &GameState,
        mut repository: HypothesisRepository<TLog>,
    ) -> HypothesisEvaluation
    where
        TLog: Log,
    {
        match &self.sub_reference {
            Some(sub_reference) => match sub_reference {
                SubReferenceType::CheckTestimony(testimony_expression_hypothesis) => {
                    let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);
                    let result = evaluator.sub_evaluate(testimony_expression_hypothesis);
                    evaluator.finalize(result)
                }
                SubReferenceType::GetTestimony(desire_producer_reference) => {
                    repository.set_desire(desire_producer_reference, true);
                    repository.finalize(HypothesisResult::Conclusive(FitnessAndAction::new(
                        FITNESS_UNKNOWN,
                        None,
                    )))
                }
            },
            None => {
                // villager is confirmed

                if let Villager::Confirmed(confirmed_villager) = game_state.villager(&self.index) {
                    let truthful = !confirmed_villager.lies();

                    repository.finalize(HypothesisResult::Conclusive(if truthful {
                        FitnessAndAction::certainty(None)
                    } else {
                        FitnessAndAction::impossible()
                    }))
                } else {
                    panic!("Villager {} should've been confirmed!", self.index)
                }
            }
        }
    }
}
