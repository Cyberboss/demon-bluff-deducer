use std::{cell::RefCell, collections::HashSet};

use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::hypotheses::HypothesisType;

use super::{
    HypothesisReference,
    cycle::Cycle,
    dependencies::DependencyData,
    depth::Depth,
    desire::{Desire, DesireData, DesireDefinition},
    hypothesis::Hypothesis,
    iteration_data::IterationData,
    misc::GraphBuilder,
};

#[derive(Debug)]
pub(super) struct StackData<'a, TLog, THypothesis, TDesire>
where
    TLog: Log,
    THypothesis: Hypothesis,
    TDesire: Desire,
{
    reference_stack: Vec<HypothesisReference>,
    pub log: &'a TLog,
    pub game_state: &'a GameState,
    pub cycles: &'a RefCell<HashSet<Cycle>>,
    hypotheses: &'a Vec<RefCell<THypothesis>>,
    pub previous_data: Option<&'a IterationData>,
    current_data: &'a RefCell<IterationData>,
    graph_builder: Option<&'a RefCell<GraphBuilder>>,
    break_at: &'a Option<HypothesisReference>,
    desire_definitions: &'a Vec<DesireDefinition<TDesire>>,
    desire_data: &'a RefCell<Vec<DesireData>>,
    dependencies: &'a DependencyData,
}

impl<'a, TLog, THypothesis, TDesire> StackData<'a, TLog, THypothesis, TDesire>
where
    TLog: Log,
    THypothesis: Hypothesis,
    TDesire: Desire,
{
    pub fn new(
        game_state: &'a GameState,
        log: &'a TLog,
        hypotheses: &'a Vec<RefCell<HypothesisType>>,
        cycles: &'a RefCell<HashSet<Cycle>>,
        previous_data: Option<&'a IterationData>,
        current_data: &'a RefCell<IterationData>,
        break_at: &'a Option<HypothesisReference>,
        root_reference: &HypothesisReference,
        graph_builder: Option<&'a RefCell<GraphBuilder>>,
        desire_definitions: &'a Vec<DesireDefinition<TDesire>>,
        desire_data: &'a RefCell<Vec<DesireData>>,
        dependencies: &'a DependencyData,
    ) -> Self {
        Self {
            reference_stack: vec![root_reference.clone()],
            log,
            game_state,
            previous_data,
            hypotheses,
            current_data,
            break_at,
            cycles,
            graph_builder,
            desire_definitions,
            desire_data,
            dependencies,
        }
    }

    pub fn create_cycle(&self, locked_reference: &HypothesisReference) -> Cycle {
        let mut order_from_root = Vec::new();
        order_from_root.push(locked_reference.clone());
        let mut adding = false;
        for trace_reference in &self.reference_stack {
            if adding {
                order_from_root.push(trace_reference.clone());
            } else if trace_reference == locked_reference {
                adding = true;
            }
        }

        Cycle::new(order_from_root)
    }

    pub fn share(&self) -> Self {
        let mut reference_stack = Vec::with_capacity(self.reference_stack.len());
        for mapped_reference in self
            .reference_stack
            .iter()
            .map(|reference| reference.clone())
        {
            reference_stack.push(mapped_reference);
        }
        Self {
            reference_stack,
            log: &self.log,
            game_state: &self.game_state,
            hypotheses: &self.hypotheses,
            current_data: &self.current_data,
            break_at: &self.break_at,
            graph_builder: self.graph_builder,
            previous_data: self.previous_data,
            cycles: &self.cycles,
            desire_definitions: self.desire_definitions,
            dependencies: self.dependencies,
            desire_data: self.desire_data,
        }
    }

    pub fn push(&self, new_reference: HypothesisReference) -> Self {
        let mut reference_stack = Vec::with_capacity(self.reference_stack.len() + 1);
        for mapped_reference in self
            .reference_stack
            .iter()
            .map(|reference| reference.clone())
        {
            reference_stack.push(mapped_reference);
        }

        reference_stack.push(new_reference);

        Self {
            reference_stack,
            log: &self.log,
            game_state: &self.game_state,
            hypotheses: &self.hypotheses,
            current_data: &self.current_data,
            break_at: &self.break_at,
            graph_builder: self.graph_builder,
            previous_data: self.previous_data,
            cycles: &self.cycles,
            desire_definitions: self.desire_definitions,
            dependencies: self.dependencies,
            desire_data: self.desire_data,
        }
    }

    pub fn current_reference(&self) -> &HypothesisReference {
        self.reference_stack
            .last()
            .expect("StackData should have at least one in stack!")
    }

    pub fn depth(&self) -> Depth {
        let reference = self.current_reference().clone();
        Depth::new(self.reference_stack.len() - 1, reference);
    }
}
