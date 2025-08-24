use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{Villager, VillagerIndex},
};
use log::{Log, info};

use crate::{
    hypotheses::{HypothesisType, testimony_expression::TestimonyExpressionHypothesisBuilder},
    hypothesis::{
        Depth, FITNESS_UNKNOWN, FitnessAndAction, Hypothesis, HypothesisBuilder,
        HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisResult,
        HypothesisReturn,
    },
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct IsTruthfulHypothesisBuilder {
    index: VillagerIndex,
}

#[derive(Debug)]
pub struct IsTruthfulHypothesis {
    index: VillagerIndex,
    testimony_expression_hypothesis: Option<HypothesisReference>,
}

impl IsTruthfulHypothesisBuilder {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl HypothesisBuilder for IsTruthfulHypothesisBuilder {
    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let testimony_expression_hypothesis = match game_state.villager(&self.index) {
            Villager::Active(active_villager) => match active_villager.instance().testimony() {
                Some(testimony) => Some(registrar.register(
                    TestimonyExpressionHypothesisBuilder::new(
                        self.index.clone(),
                        testimony.clone(),
                    ),
                )),
                None => None,
            },
            Villager::Hidden(_) | Villager::Confirmed(_) => None,
        };

        IsTruthfulHypothesis {
            index: self.index,
            testimony_expression_hypothesis,
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
        log: &TLog,
        _: Depth,
        _: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        match &self.testimony_expression_hypothesis {
            Some(testimony_expression_hypothesis) => {
                let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);
                let result = evaluator.sub_evaluate(testimony_expression_hypothesis);
                evaluator.create_return(result)
            }
            None => {
                info!(logger: log, "Cannot evaluate if {} is truthful because they have no testimony!", self.index);
                repository.create_return(HypothesisResult::Conclusive(FitnessAndAction::new(
                    FITNESS_UNKNOWN,
                    None,
                )))
            }
        }
    }
}
