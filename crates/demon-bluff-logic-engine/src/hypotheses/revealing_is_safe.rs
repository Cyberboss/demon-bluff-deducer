use std::collections::HashMap;

use demon_bluff_gameplay_engine::{
    affect::{Affect, NightEffect},
    game_state::{DAYS_BEFORE_NIGHT, GameState},
    villager::{Villager, VillagerArchetype, VillagerIndex},
};
use log::{Log, error};

use crate::{
    engine::{
        Depth, FITNESS_UNKNOWN, FitnessAndAction, Hypothesis, HypothesisBuilder,
        HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisResult,
        HypothesisReturn, and_fitness, or_result,
    },
    hypotheses::HypothesisType,
};

use super::archetype_in_play::ArchetypeInPlayHypothesisBuilder;

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct RevealingIsSafeHypothesisBuilder {}

#[derive(Debug)]
pub struct RevealingIsSafeHypothesis {
    unsafe_reveal_archetype_presence_hypotheses: HashMap<VillagerArchetype, HypothesisReference>,
}

impl HypothesisBuilder for RevealingIsSafeHypothesisBuilder {
    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let mut unsafe_reveal_archetype_presence_hypotheses = HashMap::new();
        for archetype in VillagerArchetype::iter() {
            // index doesn't matter for this question
            if let Some(affect) = position_unaware_affect(game_state, &archetype)
                && let Some(_) = affect_makes_revealing_unsafe(&affect, game_state)
            {
                unsafe_reveal_archetype_presence_hypotheses.insert(
                    archetype.clone(),
                    registrar.register(ArchetypeInPlayHypothesisBuilder::new(archetype)),
                );
            }
        }

        RevealingIsSafeHypothesis {
            unsafe_reveal_archetype_presence_hypotheses,
        }
        .into()
    }
}

impl Hypothesis for RevealingIsSafeHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Revealing Villagers is Safe")
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
        let mut all_hidden = true;
        for villager in game_state.villagers() {
            match villager {
                Villager::Hidden(_) => {}
                Villager::Active(_) | Villager::Confirmed(_) => {
                    all_hidden = false;
                    break;
                }
            }
        }

        // first reveal is always required, so call it safe even if it isn't
        if all_hidden {
            return repository.create_return(HypothesisResult::Conclusive(
                FitnessAndAction::certainty(None),
            ));
        }

        if self.unsafe_reveal_archetype_presence_hypotheses.is_empty() {
            error!(logger: log, "{} Found no archetypes that could make revealing unsafe? Bug?", depth);
            return repository.create_return(HypothesisResult::Conclusive(FitnessAndAction::new(
                FITNESS_UNKNOWN,
                None,
            )));
        }

        let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);
        let mut result = None;
        for (archetype, archetype_hypothesis) in &self.unsafe_reveal_archetype_presence_hypotheses {
            let new_result = evaluator.sub_evaluate(archetype_hypothesis).map(|fitness| {
                and_fitness(
                    fitness,
                    FitnessAndAction::new(
                        affect_makes_revealing_unsafe(
                            &position_unaware_affect(game_state, archetype)
                                .expect("Affect disappeared?"),
                            game_state,
                        )
                        .expect("Affect no longer makes revealing unsafe?"),
                        None,
                    ),
                )
            });
            result = Some(match result {
                Some(old_result) => or_result(old_result, new_result),
                None => new_result,
            })
        }

        evaluator.create_return(result.expect("Dumb logic error in RevealingIsSafeHypothesis"))
    }
}

fn position_unaware_affect(
    game_state: &GameState,
    archetype: &VillagerArchetype,
) -> Option<Affect> {
    archetype.affect(game_state.total_villagers(), VillagerIndex(0))
}

/// If the given [`Affect`] makes revealing unsafe returns the safety probabilty
fn affect_makes_revealing_unsafe(affect: &Affect, game_state: &GameState) -> Option<f64> {
    // TODO: Maybe revisit this implementation
    match affect {
        Affect::Corrupt(_) | Affect::Puppet(_) | Affect::DupeVillager | Affect::FakeOutcast => None,
        // revealing is NOT safe. That's all we're here to say
        Affect::BlockLastNReveals(_) => Some(0.0),
        Affect::Night(night_effect) => {
            match game_state.current_day() {
                Some(current_day) => {
                    let actions_remaining_before_night = DAYS_BEFORE_NIGHT - current_day;

                    match night_effect {
                        NightEffect::KillUnrevealed => {
                            // if we have more days available than remaining reveals, this is safe
                            let revealable_villager_count = game_state
                                .villagers()
                                .iter()
                                .filter(|villager| {
                                    if let Villager::Hidden(hidden_villager) = villager
                                        && !hidden_villager.cant_reveal()
                                    {
                                        true
                                    } else {
                                        false
                                    }
                                })
                                .count();

                            if actions_remaining_before_night as usize <= revealable_villager_count
                            {
                                None
                            } else {
                                // revealing is NOT safe. That's all we're here to say
                                Some(0.0)
                            }
                        }
                    }
                }
                None => None,
            }
        }
    }
}
