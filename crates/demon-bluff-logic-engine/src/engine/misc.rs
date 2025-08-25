use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};

use force_graph::{DefaultNodeIdx, ForceGraph};

use super::{HypothesisReference, hypothesis::HypothesisResult};

pub(super) struct GraphBuilder {
    graph: ForceGraph<GraphNodeData>,
    node_map: HashMap<HypothesisReference, DefaultNodeIdx>,
}

#[derive(Debug)]
pub struct GraphNodeData {
    description: String,
    current_fitness: Option<f64>,
}

impl Debug for GraphBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphBuilder")
            //.field("graph", &self.graph)
            .field("node_map", &self.node_map)
            .finish()
    }
}

pub fn decide_result(lhs: HypothesisResult, rhs: HypothesisResult) -> HypothesisResult {
    if lhs.fitness_and_action().fitness > rhs.fitness_and_action().fitness {
        lhs
    } else {
        rhs
    }
}

pub fn and_result(lhs: HypothesisResult, rhs: HypothesisResult) -> HypothesisResult {
    let new_fitness_and_action;
    let must_be_pending;
    match lhs {
        HypothesisResult::Pending(fitness_and_action) => {
            must_be_pending = true;
            new_fitness_and_action = fitness_and_action
        }
        HypothesisResult::Conclusive(fitness_and_action) => {
            must_be_pending = false;
            new_fitness_and_action = fitness_and_action
        }
    }
    match rhs {
        HypothesisResult::Pending(current_fitness_and_action) => HypothesisResult::Pending(
            and_fitness(current_fitness_and_action, new_fitness_and_action),
        ),
        HypothesisResult::Conclusive(current_fitness_and_action) => {
            let merged = and_fitness(current_fitness_and_action, new_fitness_and_action);

            if must_be_pending {
                HypothesisResult::Pending(merged)
            } else {
                HypothesisResult::Conclusive(merged)
            }
        }
    }
}

pub fn or_result(lhs: HypothesisResult, rhs: HypothesisResult) -> HypothesisResult {
    let new_fitness_and_action;
    let must_be_pending;
    match lhs {
        HypothesisResult::Pending(fitness_and_action) => {
            must_be_pending = true;
            new_fitness_and_action = fitness_and_action
        }
        HypothesisResult::Conclusive(fitness_and_action) => {
            must_be_pending = false;
            new_fitness_and_action = fitness_and_action
        }
    }
    match rhs {
        HypothesisResult::Pending(current_fitness_and_action) => HypothesisResult::Pending(
            or_fitness(current_fitness_and_action, new_fitness_and_action),
        ),
        HypothesisResult::Conclusive(current_fitness_and_action) => {
            let merged = or_fitness(current_fitness_and_action, new_fitness_and_action);

            if must_be_pending {
                HypothesisResult::Pending(merged)
            } else {
                HypothesisResult::Conclusive(merged)
            }
        }
    }
}

pub fn average_result(results: impl Iterator<Item = HypothesisResult>) -> Option<HypothesisResult> {
    let mut fitness_sum = 0.0;
    let mut total_items: usize = 0;
    let mut pending = false;

    for result in results {
        let fitness = match result {
            HypothesisResult::Pending(fitness_and_action) => {
                pending = true;
                fitness_and_action
            }
            HypothesisResult::Conclusive(fitness_and_action) => fitness_and_action,
        };

        fitness_sum = fitness_sum + fitness.fitness;
        total_items = total_items + 1;
    }

    if total_items == 0 {
        None
    } else {
        let average_fitness = fitness_sum / (total_items as f64);
        Some(if pending {
            HypothesisResult::Pending(FitnessAndAction::new(average_fitness, None))
        } else {
            HypothesisResult::Conclusive(FitnessAndAction::new(average_fitness, None))
        })
    }
}

pub fn fittest_result(lhs: HypothesisResult, rhs: HypothesisResult) -> HypothesisResult {
    let new_fitness_and_action;
    let must_be_pending;
    match lhs {
        HypothesisResult::Pending(fitness_and_action) => {
            must_be_pending = true;
            new_fitness_and_action = fitness_and_action
        }
        HypothesisResult::Conclusive(fitness_and_action) => {
            must_be_pending = false;
            new_fitness_and_action = fitness_and_action
        }
    }
    match rhs {
        HypothesisResult::Pending(current_fitness_and_action) => HypothesisResult::Pending(
            if current_fitness_and_action.fitness > new_fitness_and_action.fitness {
                current_fitness_and_action
            } else {
                new_fitness_and_action
            },
        ),
        HypothesisResult::Conclusive(current_fitness_and_action) => {
            let fittest = if current_fitness_and_action.fitness > new_fitness_and_action.fitness {
                current_fitness_and_action
            } else {
                new_fitness_and_action
            };

            if must_be_pending {
                HypothesisResult::Pending(fittest)
            } else {
                HypothesisResult::Conclusive(fittest)
            }
        }
    }
}

pub fn and_fitness(mut lhs: FitnessAndAction, rhs: FitnessAndAction) -> FitnessAndAction {
    for rh_action in rhs.action {
        lhs.action.insert(rh_action);
    }

    // P(A and B) = P(A) * P(B)
    lhs.fitness = lhs.fitness * rhs.fitness;
    lhs
}

pub fn or_fitness(mut lhs: FitnessAndAction, rhs: FitnessAndAction) -> FitnessAndAction {
    for rh_action in rhs.action {
        lhs.action.insert(rh_action);
    }

    // P(A or B) = P(A) + P(B) - P(A and B)
    lhs.fitness = lhs.fitness + rhs.fitness - (lhs.fitness * rhs.fitness);
    lhs
}
