use std::{cell::RefCell, collections::HashSet, fmt::Debug};

use demon_bluff_gameplay_engine::game_state::GameState;

use crate::player_action::PlayerAction;

#[derive(Debug)]
pub struct HypothesisReference(usize);

pub struct HypothesisRepository {}

pub struct HypothesisEvaluator {}

pub enum EvaluationRequestResult {
    Approved(HypothesisEvaluator),
    BreakCycle(HypothesisEvaluator),
}

pub enum HypothesisResult {
    Weak(FitnessAndAction),
    Conclusive(FitnessAndAction),
}

pub struct FitnessAndAction {
    action: HashSet<PlayerAction>,
    fitness: f64,
}

pub struct HypothesisContainer {
    hypothesis: RefCell<Box<dyn Hypothesis>>,
    last_evaluate: Option<FitnessAndAction>,
}

pub trait Hypothesis: Debug {
    fn evaluate(
        &mut self,
        game_state: &GameState,
        repository: &mut HypothesisRepository,
    ) -> HypothesisResult;
}

pub struct HypothesisRegistrar {
    hypotheses: Vec<RefCell<Box<dyn Hypothesis>>>,
}

impl FitnessAndAction {
    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    pub fn action(&self) -> &HashSet<PlayerAction> {
        &self.action
    }
}

impl HypothesisRepository {
    pub fn request_sub_evaluation(&mut self, current_fitness: f64) -> EvaluationRequestResult {
        todo!()
    }
}

impl HypothesisEvaluator {
    pub fn sub_evaluate(&mut self, hypothesis_reference: &HypothesisReference) -> HypothesisResult {
        todo!()
    }
}

impl HypothesisRegistrar {
    pub fn register(&mut self, hypothesis: HypothesisContainer) -> HypothesisReference {
        todo!()
    }
}

impl HypothesisResult {
    pub fn conclusive(fitness: f64, action: PlayerAction) -> Self {
        let mut set = HashSet::new();
        set.insert(action);
        Self::Conclusive(FitnessAndAction {
            action: set,
            fitness,
        })
    }
}

impl HypothesisContainer {
    pub fn new<HypothesisImpl>(hypothesis: HypothesisImpl) -> Self
    where
        HypothesisImpl: Hypothesis + 'static,
    {
        Self {
            hypothesis: RefCell::new(Box::new(hypothesis)),
            last_evaluate: None,
        }
    }
}
