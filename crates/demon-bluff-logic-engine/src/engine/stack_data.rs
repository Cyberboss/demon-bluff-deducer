use std::{cell::RefCell, collections::HashSet, fmt::Display};

use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use super::{
	Breakpoint, HypothesisReference, IndexReference,
	cycle::{Cycle, new_cycle},
	debugger::DebuggerData,
	dependencies::DependencyData,
	depth::Depth,
	desire::{Desire, DesireData, DesireDefinition},
	hypothesis::Hypothesis,
	iteration_data::IterationData,
};

#[derive(Debug)]
pub struct StackData<'a, TLog, THypothesis, TDesire, FDebugBreak>
where
	TLog: Log,
	THypothesis: Hypothesis,
	TDesire: Desire + Display,
	FDebugBreak: FnMut(Breakpoint) + Clone,
{
	reference_stack: Vec<HypothesisReference>,
	pub log: &'a TLog,
	pub game_state: &'a GameState,
	pub cycles: &'a RefCell<HashSet<Cycle>>,
	pub hypotheses: &'a Vec<RefCell<THypothesis>>,
	pub previous_data: Option<&'a IterationData>,
	pub current_data: &'a RefCell<IterationData>,
	pub debugger: Option<DebuggerData<FDebugBreak>>,
	pub break_at: &'a Option<HypothesisReference>,
	pub desire_definitions: &'a Vec<DesireDefinition<TDesire>>,
	pub desire_data: &'a RefCell<Vec<DesireData>>,
	pub dependencies: &'a DependencyData,
}

impl<'a, TLog, THypothesis, TDesire, FDebugBreak>
	StackData<'a, TLog, THypothesis, TDesire, FDebugBreak>
where
	TLog: Log,
	THypothesis: Hypothesis,
	TDesire: Desire + Display,
	FDebugBreak: FnMut(Breakpoint) + Clone,
{
	pub fn new(
		game_state: &'a GameState,
		log: &'a TLog,
		hypotheses: &'a Vec<RefCell<THypothesis>>,
		cycles: &'a RefCell<HashSet<Cycle>>,
		previous_data: Option<&'a IterationData>,
		current_data: &'a RefCell<IterationData>,
		break_at: &'a Option<HypothesisReference>,
		root_reference: &HypothesisReference,
		debugger: Option<DebuggerData<FDebugBreak>>,
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
			debugger,
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

		new_cycle(order_from_root)
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
			log: self.log,
			game_state: self.game_state,
			hypotheses: self.hypotheses,
			current_data: self.current_data,
			break_at: self.break_at,
			debugger: self.debugger.clone(),
			previous_data: self.previous_data,
			cycles: self.cycles,
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
			log: self.log,
			game_state: self.game_state,
			hypotheses: self.hypotheses,
			current_data: self.current_data,
			break_at: self.break_at,
			debugger: self.debugger.clone(),
			previous_data: self.previous_data,
			cycles: self.cycles,
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
		Depth::new(self.reference_stack.len() - 1, Some(reference))
	}
}
